// Example 20: TCP echo server — async networking with TcpListener/TcpStream
// Run: cargo run --example 20_tcp_echo

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 64];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buf[..n]);
    println!("  server received: {}", request.trim());

    let response = format!("echo: {}", request.trim());
    stream.write_all(response.as_bytes()).await?;
    stream.shutdown().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("=== TCP echo server ===\n");

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    println!("listening on {}", addr);

    let server = tokio::spawn(async move {
        let (stream, peer) = listener.accept().await.unwrap();
        println!("  accepted connection from {}", peer);
        handle_client(stream).await.unwrap();
    });

    // Client connects and sends one message
    let mut client = TcpStream::connect(addr).await?;
    client.write_all(b"hello tokio").await?;

    let mut response = String::new();
    let mut buf = [0u8; 128];
    loop {
        let n = client.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        response.push_str(&String::from_utf8_lossy(&buf[..n]));
    }

    server.await.unwrap();
    println!("client received: {}", response.trim());

    println!("\nTokio wraps non-blocking I/O: accept/read/write are all async.");
    Ok(())
}
