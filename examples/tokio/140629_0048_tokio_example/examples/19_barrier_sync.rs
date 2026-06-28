// Example 19: tokio::sync::Barrier — synchronize tasks at a checkpoint
// Run: cargo run --example 19_barrier_sync

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Barrier;

async fn pipeline_stage(name: &str, work_ms: u64, barrier: Arc<Barrier>) {
    println!("  {} working...", name);
    tokio::time::sleep(Duration::from_millis(work_ms)).await;
    println!("  {} reached barrier", name);

    barrier.wait().await;
    println!("  {} continuing after sync", name);
}

#[tokio::main]
async fn main() {
    println!("=== tokio::sync::Barrier ===\n");

    // 3 workers + main thread will wait at the barrier
    let barrier = Arc::new(Barrier::new(3));

    let h1 = {
        let barrier = Arc::clone(&barrier);
        tokio::spawn(async move {
            pipeline_stage("extract", 100, barrier).await;
        })
    };
    let h2 = {
        let barrier = Arc::clone(&barrier);
        tokio::spawn(async move {
            pipeline_stage("transform", 200, barrier).await;
        })
    };
    let h3 = {
        let barrier = Arc::clone(&barrier);
        tokio::spawn(async move {
            pipeline_stage("load", 150, barrier).await;
        })
    };

    h1.await.unwrap();
    h2.await.unwrap();
    h3.await.unwrap();

    println!("\nBarrier waits until all participants arrive before releasing anyone.");
}
