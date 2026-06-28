// Example 30: Production-style TCP server — capstone combining advanced patterns
// Run: cargo run --example 30_production_tcp_server
//
// This capstone integrates patterns from examples 21–29:
//   • TcpListener accept loop
//   • Semaphore for max connections
//   • CancellationToken for graceful shutdown
//   • JoinSet to track in-flight handlers
//   • Per-connection timeout
//   • Simple HTTP-like routing (GET /health, GET /echo?msg=...)
//   • Structured logging-style output

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

const MAX_CONNECTIONS: usize = 4;
const READ_TIMEOUT: Duration = Duration::from_secs(2);
const MAX_REQUEST_BYTES: usize = 1024;

struct ServerConfig {
    read_timeout: Duration,
}

struct ServerMetrics {
    total_requests: std::sync::atomic::AtomicU64,
    health_checks: std::sync::atomic::AtomicU64,
    echo_requests: std::sync::atomic::AtomicU64,
    errors: std::sync::atomic::AtomicU64,
}

impl ServerMetrics {
    fn snapshot(&self) -> (u64, u64, u64, u64) {
        use std::sync::atomic::Ordering;
        (
            self.total_requests.load(Ordering::Relaxed),
            self.health_checks.load(Ordering::Relaxed),
            self.echo_requests.load(Ordering::Relaxed),
            self.errors.load(Ordering::Relaxed),
        )
    }
}

struct AppState {
    config: ServerConfig,
    metrics: Arc<ServerMetrics>,
    conn_limit: Arc<Semaphore>,
    shutdown: CancellationToken,
}

impl AppState {
    fn new(shutdown: CancellationToken) -> Arc<Self> {
        Arc::new(Self {
            config: ServerConfig {
                read_timeout: READ_TIMEOUT,
            },
            metrics: Arc::new(ServerMetrics {
                total_requests: Default::default(),
                health_checks: Default::default(),
                echo_requests: Default::default(),
                errors: Default::default(),
            }),
            conn_limit: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
            shutdown,
        })
    }
}

fn parse_http_request(raw: &str) -> Option<(&str, &str)> {
    let first_line = raw.lines().next()?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next()?;
    let path = parts.next()?;
    Some((method, path))
}

fn build_response(status: u16, reason: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\nConnection: close\r\nContent-Type: text/plain\r\n\r\n{body}",
        body.len()
    )
}

fn route_request(method: &str, path: &str, state: &AppState) -> String {
    use std::sync::atomic::Ordering;

    state.metrics.total_requests.fetch_add(1, Ordering::Relaxed);

    match (method, path) {
        ("GET", "/health") => {
            state.metrics.health_checks.fetch_add(1, Ordering::Relaxed);
            let (total, health, echo, errors) = state.metrics.snapshot();
            let body = format!("ok\ntotal={total} health={health} echo={echo} errors={errors}\n");
            build_response(200, "OK", &body)
        }
        ("GET", path) if path.starts_with("/echo?msg=") => {
            state.metrics.echo_requests.fetch_add(1, Ordering::Relaxed);
            let msg = path.trim_start_matches("/echo?msg=");
            build_response(200, "OK", msg)
        }
        ("GET", "/metrics") => {
            let (total, health, echo, errors) = state.metrics.snapshot();
            let body = format!("requests={total}\nhealth={health}\necho={echo}\nerrors={errors}\n");
            build_response(200, "OK", &body)
        }
        _ => {
            state.metrics.errors.fetch_add(1, Ordering::Relaxed);
            build_response(404, "Not Found", "unknown route\n")
        }
    }
}

async fn handle_connection(stream: TcpStream, state: Arc<AppState>) -> std::io::Result<()> {
    let peer = stream.peer_addr()?;
    let started = Instant::now();

    let _permit = state.conn_limit.acquire().await.unwrap();
    println!("  [{peer}] connection accepted (active limit: {})", MAX_CONNECTIONS);

    let read_future = async {
        let mut stream = stream;
        let mut buf = vec![0u8; MAX_REQUEST_BYTES];
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok::<(), std::io::Error>(());
        }

        let raw = String::from_utf8_lossy(&buf[..n]);
        let response = match parse_http_request(&raw) {
            Some((method, path)) => {
                println!("  [{peer}] {method} {path}");
                route_request(method, path, &state)
            }
            None => build_response(400, "Bad Request", "malformed HTTP\n"),
        };

        stream.write_all(response.as_bytes()).await?;
        stream.shutdown().await?;
        Ok(())
    };

    match timeout(state.config.read_timeout, read_future).await {
        Ok(Ok(())) => println!("  [{peer}] done in {:?}", started.elapsed()),
        Ok(Err(e)) => eprintln!("  [{peer}] io error: {e}"),
        Err(_) => eprintln!("  [{peer}] timed out after {:?}", state.config.read_timeout),
    }

    Ok(())
}

async fn run_server(listener: TcpListener, state: Arc<AppState>) {
    let mut set = JoinSet::new();

    loop {
        tokio::select! {
            biased;

            _ = state.shutdown.cancelled() => {
                println!("server: shutdown signal — stop accepting");
                break;
            }

            accept = listener.accept() => {
                match accept {
                    Ok((stream, _)) => {
                        let state = Arc::clone(&state);
                        set.spawn(async move {
                            let _ = handle_connection(stream, state).await;
                        });
                    }
                    Err(e) => eprintln!("server: accept error: {e}"),
                }
            }
        }
    }

    println!("server: draining {} in-flight connections", set.len());
    while let Some(result) = set.join_next().await {
        if let Err(e) = result {
            eprintln!("server: handler error: {e}");
        }
    }
}

async fn http_get(addr: std::net::SocketAddr, path: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(addr).await?;
    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await?;

    let mut buf = vec![0u8; 2048];
    let n = stream.read(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf[..n]).into_owned())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("=== Production-style TCP server ===\n");

    let shutdown = CancellationToken::new();
    let state = AppState::new(shutdown.clone());

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    println!("server listening on {addr}");
    println!("routes: GET /health  GET /echo?msg=...  GET /metrics\n");

    let server = tokio::spawn(run_server(listener, Arc::clone(&state)));

    // Simulate client traffic
    let c1 = http_get(addr, "/health");
    let c2 = http_get(addr, "/echo?msg=hello-production");
    let c3 = http_get(addr, "/metrics");
    let c4 = http_get(addr, "/unknown");

    let (r1, r2, r3, r4) = tokio::join!(c1, c2, c3, c4);

    for (label, result) in [
        ("health", r1),
        ("echo", r2),
        ("metrics", r3),
        ("404", r4),
    ] {
        match result {
            Ok(resp) => {
                let body = resp.split("\r\n\r\n").nth(1).unwrap_or("").trim();
                println!("client {label}: {body}");
            }
            Err(e) => println!("client {label} error: {e}"),
        }
    }

    // Graceful shutdown
    println!("\nmain: triggering graceful shutdown");
    shutdown.cancel();
    server.await.unwrap();

    let (total, health, echo, errors) = state.metrics.snapshot();
    println!("\nfinal metrics: total={total} health={health} echo={echo} errors={errors}");
    println!("\nThis pattern scales to real services: add TLS, middleware, tracing, etc.");
    Ok(())
}
