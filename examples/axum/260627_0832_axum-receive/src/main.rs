use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
    signal,
    sync::Mutex,
};
use tower_http::cors::{Any, CorsLayer};

const DEFAULT_PORT: u16 = 8202;
const DEFAULT_LOG_FILE: &str = "received.jsonl";

#[derive(Clone, Serialize, Deserialize)]
struct Payload {
    stream: String,
    sequence: u64,
    interval_ms: u64,
    value: f64,
    timestamp: u64,
}

#[derive(Serialize)]
struct LogEntry {
    received_at: u64,
    payload: Payload,
}

#[derive(Serialize)]
struct IngestResponse {
    ok: bool,
    received_at: u64,
    total_received: u64,
    log_file: String,
}

#[derive(Serialize)]
struct HealthResponse {
    ok: bool,
    service: &'static str,
    total_received: u64,
    log_file: String,
}

struct AppState {
    log_file: PathBuf,
    writer: Mutex<File>,
    total_received: AtomicU64,
}

fn port() -> u16 {
    std::env::var("EXAMPLE_AXUM_RECEIVE_PORT")
        .or_else(|_| std::env::var("PORT"))
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT)
}

fn log_file_path() -> PathBuf {
    std::env::var("LOG_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_LOG_FILE))
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_millis() as u64
}

async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        ok: true,
        service: "axum-receive",
        total_received: state.total_received.load(Ordering::Relaxed),
        log_file: state.log_file.display().to_string(),
    })
}

async fn ingest(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Payload>,
) -> Result<Json<IngestResponse>, StatusCode> {
    let received_at = now_ms();
    let entry = LogEntry {
        received_at,
        payload,
    };

    let line = serde_json::to_string(&entry).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut file = state.writer.lock().await;
    file.write_all(line.as_bytes())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    file.write_all(b"\n")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    file.flush().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = state.total_received.fetch_add(1, Ordering::Relaxed) + 1;
    println!(
        "[example-axum-receive] saved #{total} stream={} seq={} -> {}",
        entry.payload.stream,
        entry.payload.sequence,
        state.log_file.display()
    );

    Ok(Json(IngestResponse {
        ok: true,
        received_at,
        total_received: total,
        log_file: state.log_file.display().to_string(),
    }))
}

#[tokio::main]
async fn main() {
    let log_file = log_file_path();
    if let Some(parent) = log_file.parent() {
        if !parent.as_os_str().is_empty() {
            tokio::fs::create_dir_all(parent)
                .await
                .expect("failed to create log directory");
        }
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .await
        .expect("failed to open log file");

    let state = Arc::new(AppState {
        log_file: log_file.clone(),
        writer: Mutex::new(file),
        total_received: AtomicU64::new(0),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/ingest", post(ingest))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port()));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    println!(
        "[example-axum-receive] listening on {addr}, log={}",
        log_file.display()
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");

    println!("[example-axum-receive] stopped");
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

    println!("[example-axum-receive] shutting down...");
}
