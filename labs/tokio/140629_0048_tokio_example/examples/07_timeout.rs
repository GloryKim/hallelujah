// Example 07: tokio::time::timeout — limit how long a Future may run
// Run: cargo run --example 07_timeout

use std::time::Duration;
use tokio::time::timeout;

async fn slow_download(size_mb: u32) -> String {
    let delay = size_mb as u64 * 100;
    tokio::time::sleep(Duration::from_millis(delay)).await;
    format!("{}MB download complete", size_mb)
}

#[tokio::main]
async fn main() {
    println!("=== tokio::time::timeout ===\n");

    // timeout: Ok(result) if finished in time, Err(Elapsed) otherwise
    match timeout(Duration::from_millis(500), slow_download(3)).await {
        Ok(result) => println!("success: {}", result),
        Err(_) => println!("failed: could not download 3MB within 500ms"),
    }

    match timeout(Duration::from_millis(500), slow_download(2)).await {
        Ok(result) => println!("success: {}", result),
        Err(_) => println!("failed: timeout"),
    }

    println!("\n-- timeout with select! --");
    let work = slow_download(5);
    tokio::pin!(work);

    tokio::select! {
        result = &mut work => {
            println!("work finished: {}", result);
        }
        _ = tokio::time::sleep(Duration::from_millis(300)) => {
            println!("300ms timeout — work cancelled (Future dropped)");
        }
    }

    println!("\ntimeout() returns Err(Elapsed). select! is also useful for cancellation.");
}
