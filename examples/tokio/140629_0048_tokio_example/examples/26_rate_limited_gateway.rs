// Example 26: Rate-limited API gateway — token bucket + concurrency cap
// Run: cargo run --example 26_rate_limited_gateway
//
// Real gateways combine multiple limits:
//   1. Token bucket  — max requests per second (smoothed burst)
//   2. Semaphore     — max concurrent in-flight requests
//   3. Per-request timeout

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, Semaphore};
use tokio::time::timeout;

/// Token bucket: refill tokens at a fixed rate, consume one per request.
struct TokenBucket {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            tokens: max_tokens,
            max_tokens,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_refill = Instant::now();
    }

    fn time_until_token(&self) -> Duration {
        let deficit = 1.0 - self.tokens;
        Duration::from_secs_f64((deficit / self.refill_rate).max(0.01))
    }
}

struct RateLimiter {
    inner: Mutex<TokenBucket>,
}

impl RateLimiter {
    fn new(max_tokens: f64, refill_rate: f64) -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(TokenBucket::new(max_tokens, refill_rate)),
        })
    }

    async fn acquire(&self) {
        loop {
            let wait = {
                let mut bucket = self.inner.lock().await;
                bucket.refill();
                if bucket.tokens >= 1.0 {
                    bucket.tokens -= 1.0;
                    None
                } else {
                    Some(bucket.time_until_token())
                }
            };

            match wait {
                None => return,
                Some(d) => tokio::time::sleep(d).await,
            }
        }
    }
}

struct Gateway {
    rate_limiter: Arc<RateLimiter>,
    concurrency: Arc<Semaphore>,
    request_timeout: Duration,
}

impl Gateway {
    fn new(max_rps: f64, max_concurrent: usize, request_timeout: Duration) -> Arc<Self> {
        Arc::new(Self {
            rate_limiter: RateLimiter::new(max_rps, max_rps),
            concurrency: Arc::new(Semaphore::new(max_concurrent)),
            request_timeout,
        })
    }

    async fn handle_request(self: &Arc<Self>, id: u32) -> Result<String, &'static str> {
        self.rate_limiter.acquire().await;

        let _permit = self
            .concurrency
            .acquire()
            .await
            .map_err(|_| "gateway closed")?;

        let work = async {
            tokio::time::sleep(Duration::from_millis(80)).await;
            Ok(format!("response-{id}"))
        };

        timeout(self.request_timeout, work)
            .await
            .map_err(|_| "request timeout")?
    }
}

#[tokio::main]
async fn main() {
    println!("=== Rate-limited API gateway ===\n");

    let gateway = Gateway::new(5.0, 2, Duration::from_millis(200));
    let start = Instant::now();

    let mut handles = Vec::new();
    for id in 1..=10 {
        let gw = Arc::clone(&gateway);
        handles.push(tokio::spawn(async move {
            let req_start = Instant::now();
            match gw.handle_request(id).await {
                Ok(resp) => println!(
                    "  req-{id:02} OK  {resp}  (latency {:?})",
                    req_start.elapsed()
                ),
                Err(e) => println!("  req-{id:02} ERR {e}"),
            }
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    println!("\n10 requests through 5 rps / 2 concurrent took {:?}", start.elapsed());
    println!("\nLayer limits: rate (token bucket) + concurrency (semaphore) + timeout.");
}
