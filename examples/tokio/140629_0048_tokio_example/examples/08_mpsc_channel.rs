// Example 08: tokio::sync::mpsc — async message channel
// Run: cargo run --example 08_mpsc_channel

use std::time::Duration;

async fn producer(name: &str, tx: tokio::sync::mpsc::Sender<String>, count: u32) {
    for i in 1..=count {
        let msg = format!("[{}] message {}", name, i);
        if tx.send(msg).await.is_err() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("=== tokio::sync::mpsc ===\n");

    // mpsc: multi-producer, single-consumer
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(4);

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let p1 = tokio::spawn(async move {
        producer("A", tx1, 3).await;
    });
    let p2 = tokio::spawn(async move {
        producer("B", tx2, 2).await;
    });

    // Drop the original tx — only clones are used by producers
    drop(tx);

    println!("-- receiving --");
    while let Some(msg) = rx.recv().await {
        println!("  {}", msg);
    }

    p1.await.unwrap();
    p2.await.unwrap();

    println!("\nrecv() returns None after all senders are dropped.");

    // oneshot: single message, single response
    println!("\n-- oneshot channel --");
    let (otx, orx) = tokio::sync::oneshot::channel::<i32>();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(30)).await;
        otx.send(42).unwrap();
    });

    match orx.await {
        Ok(n) => println!("oneshot response: {}", n),
        Err(_) => println!("sender dropped"),
    }
}
