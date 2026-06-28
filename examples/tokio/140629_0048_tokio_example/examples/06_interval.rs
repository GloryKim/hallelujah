// Example 06: tokio::time::interval — periodic execution
// Run: cargo run --example 06_interval

use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};

#[tokio::main]
async fn main() {
    println!("=== tokio::time::interval ===\n");

    // interval: call tick() on a fixed schedule
    let mut ticker = interval(Duration::from_millis(500));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    println!("-- 3 ticks (500ms apart) --");
    for i in 1..=3 {
        ticker.tick().await;
        println!("  tick #{} — {:?}", i, std::time::Instant::now());
    }

    println!("\n-- heartbeat simulation --");
    let mut heartbeat = interval(Duration::from_millis(300));
    let (tx, mut rx) = tokio::sync::mpsc::channel(8);

    let sender = tokio::spawn(async move {
        for beat in 1..=5 {
            heartbeat.tick().await;
            tx.send(format!("heartbeat-{}", beat)).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("  received: {}", msg);
    }
    sender.await.unwrap();

    println!("\ninterval waits until the next period each time you await tick().");
}
