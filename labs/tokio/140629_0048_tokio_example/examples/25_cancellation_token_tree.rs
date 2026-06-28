// Example 25: CancellationToken tree — hierarchical cancellation
// Run: cargo run --example 25_cancellation_token_tree
//
// tokio_util::sync::CancellationToken lets you build a cancellation tree:
//   root.cancel()  ──► cancels all descendants
//   child.cancel() ──► cancels only that subtree
//
// This is cleaner than wiring watch channels everywhere.

use std::time::Duration;
use tokio_util::sync::CancellationToken;

async fn worker(name: &str, token: CancellationToken, work_ms: u64) {
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("  [{name}] cancelled — cleaning up");
                tokio::time::sleep(Duration::from_millis(30)).await;
                println!("  [{name}] cleanup done");
                return;
            }
            _ = tokio::time::sleep(Duration::from_millis(work_ms)) => {
                println!("  [{name}] tick");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== CancellationToken tree ===\n");

    // Build a tree: root → [api, db] → db has children [reader, writer]
    let root = CancellationToken::new();
    let api_token = root.child_token();
    let db_token = root.child_token();
    let db_reader = db_token.child_token();
    let db_writer = db_token.child_token();

    let h_api = tokio::spawn(worker("api", api_token, 100));
    let h_db_r = tokio::spawn(worker("db-reader", db_reader.clone(), 80));
    let h_db_w = tokio::spawn(worker("db-writer", db_writer.clone(), 80));

    tokio::time::sleep(Duration::from_millis(250)).await;

    // Cancel only the db subtree
    println!("main: cancelling db subtree only");
    db_token.cancel();

    tokio::time::sleep(Duration::from_millis(200)).await;
    println!("main: api still running...\n");

    // Cancel everything via root
    println!("main: cancelling root (all remaining tasks)");
    root.cancel();

    h_api.await.unwrap();
    h_db_r.await.unwrap();
    h_db_w.await.unwrap();

    println!("\nCancellationToken supports:");
    println!("  • child_token() for scoped cancellation");
    println!("  • cancelled() future for select! integration");
    println!("  • is_cancelled() for non-async checks");
}
