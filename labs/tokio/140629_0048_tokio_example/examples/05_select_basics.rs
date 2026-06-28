// Example 05: tokio::select! — handle the first completed Future
// Run: cargo run --example 05_select_basics

use std::time::Duration;

async fn slow_task(name: &str, ms: u64) -> &str {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    name
}

#[tokio::main]
async fn main() {
    println!("=== tokio::select! ===\n");

    // select!: run the branch of the first completed Future
    tokio::select! {
        result = slow_task("fast task", 50) => {
            println!("selected: {} (50ms)", result);
        }
        result = slow_task("slow task", 200) => {
            println!("selected: {} (200ms)", result);
        }
    }

    println!("\n-- channel + select! --");
    let (tx, mut rx) = tokio::sync::mpsc::channel::<&str>(4);

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(30)).await;
        tx.send("channel message").await.unwrap();
    });

    tokio::select! {
        msg = rx.recv() => {
            println!("received from channel: {:?}", msg);
        }
        _ = tokio::time::sleep(Duration::from_millis(500)) => {
            println!("timeout (500ms)");
        }
    }

    println!("\nselect! is commonly used for races, timeouts, and cancellation.");
}
