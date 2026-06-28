// Example 11: tokio::sync::broadcast — fan-out to multiple receivers
// Run: cargo run --example 11_broadcast_channel

use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("=== tokio::sync::broadcast ===\n");

    // broadcast: one sender, many receivers (each gets a copy)
    let (tx, rx1) = tokio::sync::broadcast::channel::<String>(4);
    let rx2 = tx.subscribe();
    let rx3 = tx.subscribe();

    tokio::spawn(async move {
        for event in ["login", "purchase", "logout"] {
            tx.send(format!("event: {}", event)).unwrap();
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    let reader = |name: &str, mut rx: tokio::sync::broadcast::Receiver<String>| {
        let name = name.to_string();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                println!("  [{}] {}", name, msg);
            }
        })
    };

    let h1 = reader("analytics", rx1);
    let h2 = reader("audit-log", rx2);
    let h3 = reader("metrics", rx3);

    h1.await.unwrap();
    h2.await.unwrap();
    h3.await.unwrap();

    println!("\nbroadcast delivers each message to every active subscriber.");
    println!("Late subscribers miss messages sent before they subscribed.");
}
