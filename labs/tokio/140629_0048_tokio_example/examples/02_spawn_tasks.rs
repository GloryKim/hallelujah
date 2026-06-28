// Example 02: tokio::spawn — run background tasks
// Run: cargo run --example 02_spawn_tasks

use tokio::task::JoinHandle;

async fn worker(id: u32, delay_ms: u64) -> u32 {
    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    println!("  worker {} done", id);
    id * 10
}

#[tokio::main]
async fn main() {
    println!("=== tokio::spawn ===\n");

    // spawn: start an independent task on the current runtime
    let handle: JoinHandle<u32> = tokio::spawn(async {
        worker(1, 100).await
    });

    println!("main task: spawned worker 1 (not waiting yet)");

    // JoinHandle.await — wait until the task completes
    let result = handle.await.expect("task panicked");
    println!("worker 1 result: {}\n", result);

    // Spawn multiple tasks concurrently
    println!("-- 3 tasks running concurrently --");
    let handles: Vec<_> = (1..=3)
        .map(|id| {
            tokio::spawn(async move {
                worker(id, 80).await
            })
        })
        .collect();

    for handle in handles {
        let value = handle.await.expect("task panicked");
        println!("  result: {}", value);
    }

    println!("\nspawned tasks run in parallel with main.");
}
