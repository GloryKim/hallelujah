// Example 18: tokio::sync::Notify — async condition variable
// Run: cargo run --example 18_notify_condition

use std::sync::Arc;
use tokio::sync::Notify;

async fn waiter(id: u32, notify: Arc<Notify>) {
    println!("  waiter {} waiting...", id);
    notify.notified().await;
    println!("  waiter {} woke up", id);
}

#[tokio::main]
async fn main() {
    println!("=== tokio::sync::Notify ===\n");

    let notify = Arc::new(Notify::new());

    let handles: Vec<_> = (1..=3)
        .map(|id| {
            let notify = Arc::clone(&notify);
            tokio::spawn(async move {
                waiter(id, notify).await;
            })
        })
        .collect();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    println!("main: notify_one()");
    notify.notify_one();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    println!("main: notify_waiters()");
    notify.notify_waiters();

    for handle in handles {
        handle.await.unwrap();
    }

    println!("\nNotify wakes tasks waiting on notified().");
    println!("Use it for work queues, connection pools, and custom sync primitives.");
}
