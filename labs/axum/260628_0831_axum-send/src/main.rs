use reqwest::Client;
use serde::Serialize;
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{signal, time::Duration};

const DEFAULT_RECEIVER: &str = "http://127.0.0.1:8202/ingest";
const INTERVAL_FAST_MS: u64 = 10;
const INTERVAL_SLOW_MS: u64 = 30;

#[derive(Serialize)]
struct Payload {
    stream: String,
    sequence: u64,
    interval_ms: u64,
    value: f64,
    timestamp: u64,
}

struct StreamState {
    name: &'static str,
    interval_ms: u64,
    sequence: AtomicU64,
}

fn receiver_url() -> String {
    std::env::var("RECEIVER_URL").unwrap_or_else(|_| DEFAULT_RECEIVER.to_string())
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_millis() as u64
}

fn next_payload(stream: &StreamState) -> Payload {
    let sequence = stream.sequence.fetch_add(1, Ordering::Relaxed) + 1;
    Payload {
        stream: stream.name.to_string(),
        sequence,
        interval_ms: stream.interval_ms,
        value: (sequence as f64 * 0.1).sin(),
        timestamp: now_ms(),
    }
}

async fn send_loop(client: Client, url: String, stream: Arc<StreamState>) {
    let mut interval = tokio::time::interval(Duration::from_millis(stream.interval_ms));
    interval.tick().await;

    loop {
        interval.tick().await;

        let payload = next_payload(&stream);
        match client.post(&url).json(&payload).send().await {
            Ok(resp) if resp.status().is_success() => {
                println!(
                    "[example-axum-send] ok stream={} seq={} interval={}ms",
                    payload.stream, payload.sequence, payload.interval_ms
                );
            }
            Ok(resp) => {
                eprintln!(
                    "[example-axum-send] failed stream={} status={}",
                    payload.stream,
                    resp.status()
                );
            }
            Err(err) => {
                eprintln!(
                    "[example-axum-send] error stream={} seq={}: {err}",
                    payload.stream, payload.sequence
                );
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let url = receiver_url();
    let client = Client::new();

    let fast = Arc::new(StreamState {
        name: "fast-10ms",
        interval_ms: INTERVAL_FAST_MS,
        sequence: AtomicU64::new(0),
    });
    let slow = Arc::new(StreamState {
        name: "slow-30ms",
        interval_ms: INTERVAL_SLOW_MS,
        sequence: AtomicU64::new(0),
    });

    println!(
        "[example-axum-send] sending JSON every {INTERVAL_FAST_MS}ms and {INTERVAL_SLOW_MS}ms -> {url}"
    );

    let fast_handle = {
        let client = client.clone();
        let url = url.clone();
        let stream = fast.clone();
        tokio::spawn(async move {
            send_loop(client, url, stream).await;
        })
    };

    let slow_handle = {
        let client = client.clone();
        let url = url.clone();
        let stream = slow.clone();
        tokio::spawn(async move {
            send_loop(client, url, stream).await;
        })
    };

    shutdown_signal().await;
    fast_handle.abort();
    slow_handle.abort();

    println!("[example-axum-send] stopped");
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

    println!("[example-axum-send] shutting down...");
}
