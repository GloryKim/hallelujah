// Example 22: Multi-client TCP chat server — broadcast fan-out over TCP
// Run: cargo run --example 22_tcp_chat_server
//
// Architecture:
//   Client ──TCP──► Server ──broadcast──► all other clients
//
// Each TCP connection gets a dedicated task that:
//   1. Reads lines from the client
//   2. Publishes them to a broadcast channel
//   3. Forwards messages from other clients back to this client

use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

const MAX_CLIENTS: usize = 16;

type ChatBus = broadcast::Sender<Arc<str>>;

async fn handle_client(
    stream: TcpStream,
    bus: ChatBus,
    client_name: Arc<str>,
) -> std::io::Result<()> {
    let peer = stream.peer_addr()?;
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut rx = bus.subscribe();

    // Task A: read from TCP, publish to bus
    let publish_name = Arc::clone(&client_name);
    let publish_bus = bus.clone();
    let reader_task = tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                break;
            }
            let msg = format!("[{}] {}", publish_name, line.trim());
            let _ = publish_bus.send(Arc::from(msg.as_str()));
        }
        Ok::<(), std::io::Error>(())
    });

    // Task B: read from bus, write to TCP
    let writer_task = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    let out = format!("{}\n", msg);
                    if writer.write_all(out.as_bytes()).await.is_err() {
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    let notice = format!("--- missed {n} messages ---\n");
                    let _ = writer.write_all(notice.as_bytes()).await;
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    println!("  client connected: {client_name} ({peer})");

    tokio::select! {
        r = reader_task => { r??; }
        _ = writer_task => {}
    }

    let leave = format!("[{}] left the chat", client_name);
    let _ = bus.send(Arc::from(leave));
    println!("  client disconnected: {client_name}");
    Ok(())
}

async fn run_chat_server(listener: TcpListener, bus: ChatBus) {
    let mut next_id = 1u32;
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let name = Arc::from(format!("user-{next_id}"));
        next_id += 1;

        let bus = bus.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, bus, name).await {
                eprintln!("  client error: {e}");
            }
        });
    }
}

async fn chat_client(
    addr: std::net::SocketAddr,
    name: &str,
    messages: &[&str],
) -> std::io::Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    writer
        .write_all(format!("joined as {name}\n").as_bytes())
        .await?;

    for msg in messages {
        writer
            .write_all(format!("says: {msg}\n").as_bytes())
            .await?;
        tokio::time::sleep(std::time::Duration::from_millis(40)).await;
    }

    // Read broadcast messages from other clients
    let mut line = String::new();
    for _ in 0..5 {
        line.clear();
        let n = timeout_read_line(&mut reader, &mut line).await?;
        if n == 0 {
            break;
        }
        print!("    client-{name} saw: {line}");
    }

    writer.shutdown().await?;
    Ok(())
}

async fn timeout_read_line(
    reader: &mut BufReader<tokio::net::tcp::OwnedReadHalf>,
    line: &mut String,
) -> std::io::Result<usize> {
    tokio::time::timeout(std::time::Duration::from_millis(300), reader.read_line(line))
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "read timeout"))?
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("=== Multi-client TCP chat server ===\n");

    let (bus, _) = broadcast::channel::<Arc<str>>(MAX_CLIENTS);
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    println!("chat server listening on {addr}\n");

    let server = tokio::spawn(run_chat_server(listener, bus));

    tokio::time::sleep(std::time::Duration::from_millis(30)).await;

    let c1 = tokio::spawn(chat_client(addr, "alice", &["hello everyone", "anyone here?"]));
    let c2 = tokio::spawn(chat_client(addr, "bob", &["hey alice!", "good morning"]));
    let c3 = tokio::spawn(chat_client(addr, "carol", &["hi team"]));

    c1.await??;
    c2.await??;
    c3.await??;

    server.abort();
    println!("\nEach client task splits read/write and uses broadcast for fan-out.");
    println!("Lagged receivers get RecvError::Lagged when they fall behind.");
    Ok(())
}
