// Example 23: Worker pool with backpressure — bounded channel + N workers
// Run: cargo run --example 23_worker_pool_backpressure
//
// When producers outpace consumers, unbounded queues grow until OOM.
// This pattern uses a bounded mpsc channel so slow workers naturally
// slow down producers (backpressure).
//
// Components:
//   Producer  ──► [bounded queue] ──► Worker 1..N
//                      │
//                      └── send().await blocks when full

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Semaphore};

#[derive(Debug, Clone)]
struct Job {
    id: u64,
    payload: String,
}

#[derive(Default)]
struct Metrics {
    enqueued: AtomicU64,
    processed: AtomicU64,
    rejected: AtomicU64,
}

struct WorkerPool {
    tx: mpsc::Sender<Job>,
    metrics: Arc<Metrics>,
    _workers: Vec<tokio::task::JoinHandle<()>>,
}

impl WorkerPool {
    fn new(worker_count: usize, queue_capacity: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Job>(queue_capacity);
        let rx = Arc::new(tokio::sync::Mutex::new(rx));
        let metrics = Arc::new(Metrics::default());
        let mut workers = Vec::with_capacity(worker_count);

        for worker_id in 0..worker_count {
            let rx = Arc::clone(&rx);
            let metrics = Arc::clone(&metrics);

            workers.push(tokio::spawn(async move {
                loop {
                    let job = {
                        let mut guard = rx.lock().await;
                        guard.recv().await
                    };

                    match job {
                        Some(job) => {
                            println!(
                                "  worker-{worker_id} processing job #{} ({})",
                                job.id, job.payload
                            );
                            tokio::time::sleep(Duration::from_millis(120)).await;
                            metrics.processed.fetch_add(1, Ordering::Relaxed);
                        }
                        None => {
                            println!("  worker-{worker_id} shutting down");
                            break;
                        }
                    }
                }
            }));
        }

        Self {
            tx,
            metrics,
            _workers: workers,
        }
    }

    async fn submit(&self, job: Job) -> Result<(), mpsc::error::SendError<Job>> {
        self.metrics.enqueued.fetch_add(1, Ordering::Relaxed);
        self.tx.send(job).await
    }

    fn metrics(&self) -> &Metrics {
        &self.metrics
    }
}

async fn producer(
    pool: &WorkerPool,
    name: &str,
    jobs: u64,
    submit_delay: Duration,
    limiter: Arc<Semaphore>,
) {
    for id in 1..=jobs {
        let _permit = limiter.acquire().await.unwrap();

        let job = Job {
            id,
            payload: format!("{name}-task-{id}"),
        };

        let start = Instant::now();
        match pool.submit(job).await {
            Ok(()) => {
                let waited = start.elapsed();
                if waited > Duration::from_millis(10) {
                    println!("  producer {name} backpressure: waited {waited:?} for slot");
                }
            }
            Err(_) => {
                pool.metrics().rejected.fetch_add(1, Ordering::Relaxed);
            }
        }

        tokio::time::sleep(submit_delay).await;
    }
}

#[tokio::main]
async fn main() {
    println!("=== Worker pool with backpressure ===\n");

    // 3 workers, queue holds at most 2 jobs → backpressure kicks in quickly
    let pool = WorkerPool::new(3, 2);
    let limiter = Arc::new(Semaphore::new(10));

    let p1 = producer(&pool, "fast", 4, Duration::from_millis(20), Arc::clone(&limiter));
    let p2 = producer(&pool, "burst", 4, Duration::from_millis(5), Arc::clone(&limiter));

    tokio::join!(p1, p2);

    // Wait for queue to drain
    tokio::time::sleep(Duration::from_millis(800)).await;

    let m = pool.metrics();
    println!("\n-- metrics --");
    println!("  enqueued:  {}", m.enqueued.load(Ordering::Relaxed));
    println!("  processed: {}", m.processed.load(Ordering::Relaxed));
    println!("  rejected:  {}", m.rejected.load(Ordering::Relaxed));

    println!("\nBounded channels push backpressure to producers.");
    println!("Combine with Semaphore to cap upstream submission rate too.");
}
