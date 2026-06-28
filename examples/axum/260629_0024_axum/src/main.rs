use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{signal, sync::Mutex, time::Duration};
use tower_http::cors::{Any, CorsLayer};

const DEFAULT_PORT: u16 = 8201;
const DEFAULT_PEER: &str = "http://127.0.0.1:8202";

#[derive(Clone, Serialize, Deserialize)]
struct Message {
    id: String,
    from: String,
    text: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
struct IngestRequest {
    from: String,
    text: String,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    timestamp: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct IngestResponse {
    ok: bool,
    stored: Message,
    reply: Message,
}

#[derive(Serialize, Deserialize)]
struct RelayRequest {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct RelayResponse {
    local: Message,
    peer: IngestResponse,
}

#[derive(Serialize)]
struct HealthResponse {
    ok: bool,
    service: &'static str,
    peer_url: String,
    message_count: usize,
    heartbeats_sent: u64,
}

#[derive(Serialize)]
struct DemoResponse {
    steps: Vec<DemoStep>,
}

#[derive(Serialize)]
struct DemoStep {
    step: u8,
    actor: &'static str,
    action: String,
    payload: serde_json::Value,
}

struct AppState {
    service: &'static str,
    peer_url: String,
    messages: Mutex<Vec<Message>>,
    client: Client,
    heartbeats_sent: AtomicU64,
}

fn port() -> u16 {
    std::env::var("EXAMPLE_AXUM_PORT")
        .or_else(|_| std::env::var("PORT"))
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT)
}

fn peer_url() -> String {
    std::env::var("PEER_URL").unwrap_or_else(|_| DEFAULT_PEER.to_string())
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_millis() as u64
}

fn new_id(prefix: &str) -> String {
    format!("{prefix}-{}", now_ms())
}

async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let count = state.messages.lock().await.len();
    Json(HealthResponse {
        ok: true,
        service: state.service,
        peer_url: state.peer_url.clone(),
        message_count: count,
        heartbeats_sent: state.heartbeats_sent.load(Ordering::Relaxed),
    })
}

async fn list_messages(State(state): State<Arc<AppState>>) -> Json<Vec<Message>> {
    Json(state.messages.lock().await.clone())
}

async fn ingest(
    State(state): State<Arc<AppState>>,
    Json(body): Json<IngestRequest>,
) -> Result<Json<IngestResponse>, StatusCode> {
    let stored = Message {
        id: body.id.unwrap_or_else(|| new_id("ingest")),
        from: body.from.clone(),
        text: body.text.clone(),
        timestamp: body.timestamp.unwrap_or_else(now_ms),
    };

    state.messages.lock().await.push(stored.clone());

    let reply = Message {
        id: new_id("reply"),
        from: state.service.to_string(),
        text: format!("ack from axum: {}", body.text.chars().rev().collect::<String>()),
        timestamp: now_ms(),
    };

    Ok(Json(IngestResponse {
        ok: true,
        stored,
        reply,
    }))
}

async fn relay(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RelayRequest>,
) -> Result<Json<RelayResponse>, (StatusCode, String)> {
    let local = Message {
        id: new_id("relay"),
        from: state.service.to_string(),
        text: body.text.clone(),
        timestamp: now_ms(),
    };
    state.messages.lock().await.push(local.clone());

    let url = format!("{}/ingest", state.peer_url.trim_end_matches('/'));
    let peer: IngestResponse = state
        .client
        .post(&url)
        .json(&IngestRequest {
            from: state.service.to_string(),
            text: body.text,
            id: Some(local.id.clone()),
            timestamp: Some(local.timestamp),
        })
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("peer request failed: {e}")))?
        .error_for_status()
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("peer returned error: {e}")))?
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("peer json decode failed: {e}")))?;

    Ok(Json(RelayResponse { local, peer }))
}

async fn demo_roundtrip(State(state): State<Arc<AppState>>) -> Result<Json<DemoResponse>, (StatusCode, String)> {
    let mut steps = Vec::new();

    steps.push(DemoStep {
        step: 1,
        actor: "axum",
        action: "POST /relay to express".into(),
        payload: serde_json::json!({ "text": "hello from axum demo" }),
    });

    let relay = relay(
        State(state.clone()),
        Json(RelayRequest {
            text: "hello from axum demo".into(),
        }),
    )
    .await?;

    steps.push(DemoStep {
        step: 2,
        actor: "express",
        action: "responded via /ingest".into(),
        payload: serde_json::to_value(relay.0.peer)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    });

    let callback_url = format!("{}/relay", state.peer_url.trim_end_matches('/'));
    steps.push(DemoStep {
        step: 3,
        actor: "axum",
        action: "ask express to relay back".into(),
        payload: serde_json::json!({ "url": callback_url, "text": "callback from axum" }),
    });

    let callback: RelayResponse = state
        .client
        .post(&callback_url)
        .json(&RelayRequest {
            text: "callback from axum".into(),
        })
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("callback failed: {e}")))?
        .error_for_status()
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("callback error status: {e}")))?
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("callback json failed: {e}")))?;

    steps.push(DemoStep {
        step: 4,
        actor: "express",
        action: "relayed back to axum /ingest".into(),
        payload: serde_json::to_value(callback)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    });

    Ok(Json(DemoResponse { steps }))
}

async fn send_heartbeat(state: Arc<AppState>) {
    let url = format!("{}/ingest", state.peer_url.trim_end_matches('/'));
    let payload = IngestRequest {
        from: state.service.to_string(),
        text: "heartbeat".into(),
        id: Some(new_id("hb")),
        timestamp: Some(now_ms()),
    };

    match state.client.post(&url).json(&payload).send().await {
        Ok(resp) if resp.status().is_success() => {
            state.heartbeats_sent.fetch_add(1, Ordering::Relaxed);
            println!("[example-axum] heartbeat ok -> express");
        }
        Ok(resp) => {
            eprintln!("[example-axum] heartbeat failed: status {}", resp.status());
        }
        Err(err) => {
            eprintln!("[example-axum] heartbeat error: {err}");
        }
    }
}

fn spawn_heartbeat(state: Arc<AppState>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        interval.tick().await;
        loop {
            interval.tick().await;
            send_heartbeat(state.clone()).await;
        }
    });
}

#[tokio::main]
async fn main() {
    let peer = peer_url();
    let state = Arc::new(AppState {
        service: "axum",
        peer_url: peer.clone(),
        messages: Mutex::new(Vec::new()),
        client: Client::new(),
        heartbeats_sent: AtomicU64::new(0),
    });

    spawn_heartbeat(state.clone());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/messages", get(list_messages))
        .route("/ingest", post(ingest))
        .route("/relay", post(relay))
        .route("/demo/roundtrip", get(demo_roundtrip))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port()));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    println!("[example-axum] listening on {addr}, peer={peer}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");

    println!("[example-axum] stopped");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    println!("[example-axum] shutting down...");
}
