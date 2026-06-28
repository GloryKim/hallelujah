// Example 24: Task supervisor — restart failed workers with exponential backoff
// Run: cargo run --example 24_task_supervisor
//
// In long-running systems, tasks crash. A supervisor:
//   1. Spawns a worker
//   2. Awaits its completion
//   3. On failure, waits (backoff) and restarts
//   4. Gives up after max_retries

use std::time::Duration;
use tokio::sync::watch;

#[derive(Debug, Clone, Copy)]
enum WorkerOutcome {
    Completed,
    Failed,
}

struct SupervisorConfig {
    max_retries: u32,
    initial_backoff: Duration,
    max_backoff: Duration,
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_backoff: Duration::from_millis(50),
            max_backoff: Duration::from_millis(400),
        }
    }
}

/// Simulates an unreliable worker. Fails twice, then succeeds.
async fn unreliable_worker(attempt: u32, mut shutdown: watch::Receiver<bool>) -> WorkerOutcome {
    println!("  worker attempt #{attempt} starting");

    loop {
        tokio::select! {
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    println!("  worker attempt #{attempt} shutdown");
                    return WorkerOutcome::Completed;
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(80)) => {
                if attempt < 3 {
                    println!("  worker attempt #{attempt} FAILED");
                    return WorkerOutcome::Failed;
                }
                println!("  worker attempt #{attempt} succeeded");
                return WorkerOutcome::Completed;
            }
        }
    }
}

async fn run_supervisor(
    name: &str,
    config: SupervisorConfig,
    mut shutdown: watch::Receiver<bool>,
) {
    let mut attempt = 0u32;
    let mut backoff = config.initial_backoff;

    loop {
        if *shutdown.borrow() {
            println!("[supervisor {name}] shutdown requested");
            break;
        }

        attempt += 1;
        let worker_shutdown = shutdown.clone();

        let handle = tokio::spawn(async move {
            unreliable_worker(attempt, worker_shutdown).await
        });
        let abort = handle.abort_handle();

        tokio::select! {
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    abort.abort();
                    println!("[supervisor {name}] aborted worker on shutdown");
                    break;
                }
            }
            result = handle => {
                match result {
                    Ok(WorkerOutcome::Completed) => {
                        println!("[supervisor {name}] worker finished successfully");
                        break;
                    }
                    Ok(WorkerOutcome::Failed) => {
                        if attempt >= config.max_retries {
                            eprintln!("[supervisor {name}] max retries reached — giving up");
                            break;
                        }
                        println!(
                            "[supervisor {name}] restarting in {:?} (attempt {}/{})",
                            backoff, attempt, config.max_retries
                        );
                        tokio::time::sleep(backoff).await;
                        backoff = (backoff * 2).min(config.max_backoff);
                    }
                    Err(e) => {
                        eprintln!("[supervisor {name}] join error: {e}");
                        break;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== Task supervisor ===\n");

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let config = SupervisorConfig::default();

    let supervisor = tokio::spawn(run_supervisor("payment-processor", config, shutdown_rx));

    // Let supervisor run through failures and recovery
    tokio::time::sleep(Duration::from_millis(1200)).await;

    shutdown_tx.send(true).unwrap();
    supervisor.await.unwrap();

    println!("\nSupervisors add resilience: crash → backoff → restart.");
    println!("Production systems often nest supervisors (supervise the supervisor).");
}
