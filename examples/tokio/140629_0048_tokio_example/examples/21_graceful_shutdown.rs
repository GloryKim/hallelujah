// Example 21: Graceful shutdown — drain in-flight work before exit
// Run: cargo run --example 21_graceful_shutdown
//
// Production services must not drop active requests when they restart.
// This example shows the full lifecycle:
//   1. Spawn connection handlers into a JoinSet
//   2. A shutdown signal (watch channel) tells handlers to finish
//   3. join_next drains handlers until all exit
//   4. Remaining tasks are aborted if a drain deadline expires

use std::time::{Duration, Instant};
use tokio::sync::watch;
use tokio::task::JoinSet;
use tokio::time::timeout;

/// Simulates a long-lived connection handler that respects shutdown.
async fn handle_connection(id: u32, mut shutdown: watch::Receiver<bool>) {
    loop {
        tokio::select! {
            biased;

            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    println!("  [conn {id}] received shutdown — finishing cleanup");
                    tokio::time::sleep(Duration::from_millis(80)).await;
                    println!("  [conn {id}] closed cleanly");
                    return;
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(120)) => {
                println!("  [conn {id}] processing request...");
            }
        }
    }
}

/// Drain all tasks in the JoinSet, with an overall deadline.
async fn drain_connections(set: &mut JoinSet<()>, deadline: Duration) {
    let start = Instant::now();

    loop {
        let remaining = deadline.saturating_sub(start.elapsed());
        if remaining.is_zero() {
            break;
        }

        match timeout(remaining, set.join_next()).await {
            Ok(Some(Ok(()))) => println!("  handler exited normally"),
            Ok(Some(Err(e))) if e.is_cancelled() => println!("  handler was aborted"),
            Ok(Some(Err(e))) => println!("  handler panicked: {e}"),
            Ok(None) => break,
            Err(_) => break,
        }
    }

    if !set.is_empty() {
        println!("  drain deadline reached — aborting {} remaining tasks", set.len());
        set.abort_all();
        while set.join_next().await.is_some() {}
    }
}

#[tokio::main]
async fn main() {
    println!("=== Graceful shutdown ===\n");

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let mut set = JoinSet::new();

    // Spawn several connection handlers
    for id in 1..=5 {
        let rx = shutdown_rx.clone();
        set.spawn(async move {
            handle_connection(id, rx).await;
        });
    }

    // Let handlers start processing
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Phase 1: signal shutdown — handlers finish current work
    println!("main: initiating graceful shutdown");
    shutdown_tx.send(true).unwrap();

    // Phase 2: drain in-flight handlers
    println!("\n-- draining connections (800ms deadline) --");
    drain_connections(&mut set, Duration::from_millis(800)).await;

    println!("\nKey takeaways:");
    println!("  • Use watch/broadcast/CancellationToken for shutdown signals");
    println!("  • Track handlers in JoinSet so you can await them all");
    println!("  • Set a drain timeout, then abort stragglers");
    println!("  • In production, also stop accepting on TcpListener when shutting down");
}
