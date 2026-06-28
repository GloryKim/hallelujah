// Example 28: Framed length-delimited protocol — tokio-util codec
// Run: cargo run --example 28_framed_protocol
//
// Raw TCP is a byte stream — messages can be split or merged.
// Length-delimited framing prepends a frame size so the receiver
// knows exactly where each message begins and ends.
//
// Stack:
//   TcpStream → Framed<LengthDelimitedCodec> → typed messages

use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Framed, LengthDelimitedCodec};

type MessageFrame = Framed<tokio::net::TcpStream, LengthDelimitedCodec>;

async fn run_server(listener: tokio::net::TcpListener) -> std::io::Result<Vec<String>> {
    let (stream, peer) = listener.accept().await?;
    println!("  server: accepted {peer}");

    let mut framed: MessageFrame = LengthDelimitedCodec::new().framed(stream);
    let mut received = Vec::new();

    while let Some(frame) = framed.next().await {
        match frame {
            Ok(bytes) => {
                let msg = String::from_utf8_lossy(&bytes).into_owned();
                println!("  server: received frame ({:>2} bytes): {msg}", bytes.len());
                received.push(msg.clone());

                let reply = format!("ACK: {msg}");
                framed.send(Bytes::from(reply)).await?;
            }
            Err(e) => {
                eprintln!("  server: decode error: {e}");
                break;
            }
        }
    }

    Ok(received)
}

async fn run_client(addr: std::net::SocketAddr, messages: &[&str]) -> std::io::Result<Vec<String>> {
    let stream = TcpStream::connect(addr).await?;
    let mut framed: MessageFrame = LengthDelimitedCodec::new().framed(stream);
    let mut replies = Vec::new();

    for msg in messages {
        framed.send(Bytes::from(msg.to_string())).await?;
        println!("  client: sent {msg}");
    }

    // Read ACK for each message
    for _ in messages {
        if let Some(frame) = framed.next().await {
            let bytes = frame?;
            let reply = String::from_utf8_lossy(&bytes).into_owned();
            println!("  client: got {reply}");
            replies.push(reply);
        }
    }

    framed.close().await?;
    Ok(replies)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("=== Framed length-delimited protocol ===\n");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    let server = tokio::spawn(run_server(listener));

    tokio::time::sleep(std::time::Duration::from_millis(30)).await;

    let messages = ["ping", "hello framed world", "binary-safe \x00\x01 data"];
    let replies = run_client(addr, &messages).await?;

    let server_received = server.await.unwrap()?;
    assert_eq!(server_received.len(), messages.len());
    assert_eq!(replies.len(), messages.len());

    println!("\nLengthDelimitedCodec handles framing so you never read partial messages.");
    println!("Other codecs: LinesCodec (text lines), BytesCodec (raw passthrough).");
    Ok(())
}
