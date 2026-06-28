// Example 10: Arc + tokio::sync::Mutex — shared state across tasks
// Run: cargo run --example 10_async_mutex

use std::sync::Arc;
use tokio::sync::Mutex;

async fn increment(counter: Arc<Mutex<u32>>, times: u32, name: &str) {
    for _ in 0..times {
        let current = {
            let mut guard = counter.lock().await;
            *guard += 1;
            *guard
        };
        println!("  {} -> {}", name, current);
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("=== Arc + tokio::sync::Mutex ===\n");

    let counter = Arc::new(Mutex::new(0u32));

    let mut handles = Vec::new();
    for name in ["Alice", "Bob", "Charlie"] {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            increment(counter, 3, name).await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let final_count = *counter.lock().await;
    println!("\nfinal count: {} (3 x 3 = 9)", final_count);

    println!("\ntokio::sync::Mutex can safely hold a lock guard across .await.");
    println!("Do not hold a std::sync::Mutex guard across .await in async fn.");
}
