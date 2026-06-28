// Example 09: spawn_blocking — run CPU-bound work on a separate thread
// Run: cargo run --example 09_spawn_blocking

use std::time::Instant;

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn heavy_computation(n: u32) -> u64 {
    fibonacci(n)
}

#[tokio::main]
async fn main() {
    println!("=== spawn_blocking ===\n");

    // Use spawn_blocking for CPU-bound work inside async code
    let start = Instant::now();

    let handle = tokio::task::spawn_blocking(move || {
        heavy_computation(35)
    });

    // Other async work can run while blocking work is in progress
    let ticker = tokio::spawn(async {
        for i in 1..=3 {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            println!("  async ticker: {}", i);
        }
    });

    let result = handle.await.expect("spawn_blocking panicked");
    ticker.await.unwrap();

    println!("\nfibonacci(35) = {}", result);
    println!("elapsed: {:?}", start.elapsed());

    println!("\nspawn_blocking runs on Tokio's blocking thread pool.");
    println!("Calling thread::sleep or heavy CPU work directly in async fn");
    println!("can stall the whole runtime — use spawn_blocking instead.");
}
