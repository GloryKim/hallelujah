// Example 14: tokio::sync::Semaphore — limit concurrent work
// Run: cargo run --example 14_semaphore_limit

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

async fn call_api(id: u32, permit: Arc<Semaphore>) {
    let _permit = permit.acquire().await.unwrap();
    println!("  request {} started", id);
    tokio::time::sleep(Duration::from_millis(150)).await;
    println!("  request {} finished", id);
}

#[tokio::main]
async fn main() {
    println!("=== tokio::sync::Semaphore ===\n");

    // Allow at most 2 concurrent API calls
    let permit = Arc::new(Semaphore::new(2));
    let start = Instant::now();

    let handles: Vec<_> = (1..=5)
        .map(|id| {
            let permit = Arc::clone(&permit);
            tokio::spawn(async move {
                call_api(id, permit).await;
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }

    println!("\n5 requests with limit 2 took {:?}", start.elapsed());
    println!("Semaphore protects downstream resources (DB pool, API quota, etc.).");
}
