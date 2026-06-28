// Example 17: task cancellation — AbortHandle and cooperative shutdown
// Run: cargo run --example 17_task_cancellation

use std::time::Duration;
use tokio::sync::watch;

async fn background_worker(mut shutdown: watch::Receiver<bool>) {
    loop {
        tokio::select! {
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    println!("  worker received shutdown signal");
                    break;
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                println!("  worker heartbeat");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== task cancellation ===\n");

    // Pattern 1: AbortHandle — hard cancellation
    let abort_handle = {
        let handle = tokio::spawn(async {
            loop {
                tokio::time::sleep(Duration::from_millis(80)).await;
                println!("  noisy task tick");
            }
        });
        handle.abort_handle()
    };

    tokio::time::sleep(Duration::from_millis(250)).await;
    abort_handle.abort();
    println!("  noisy task aborted\n");

    // Pattern 2: cooperative shutdown with watch channel
    let (tx, rx) = watch::channel(false);
    let worker = tokio::spawn(background_worker(rx));

    tokio::time::sleep(Duration::from_millis(350)).await;
    tx.send(true).unwrap();
    worker.await.unwrap();

    println!("\nAbortHandle stops a task immediately.");
    println!("Cooperative shutdown lets tasks clean up before exiting.");
}
