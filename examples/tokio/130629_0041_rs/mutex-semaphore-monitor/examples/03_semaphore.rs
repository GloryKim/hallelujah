// 예제 03: Semaphore — Mutex + Condvar 로 counting semaphore 구현
// 실행: cargo run --example 03_semaphore
//
// std::sync::Semaphore 와 동일한 개념:
// permit(허가) 단위로 동시 접근 수를 제한한다.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const MAX_CONCURRENT: usize = 3;
const WORKERS: usize = 8;

struct CountingSemaphore {
    permits: Mutex<usize>,
    available: Condvar,
}

struct SemaphorePermit<'a> {
    sem: &'a CountingSemaphore,
}

impl CountingSemaphore {
    fn new(permits: usize) -> Self {
        Self {
            permits: Mutex::new(permits),
            available: Condvar::new(),
        }
    }

    fn available_permits(&self) -> usize {
        *self.permits.lock().unwrap()
    }

    fn acquire(&self) -> SemaphorePermit<'_> {
        let mut count = self.permits.lock().unwrap();
        while *count == 0 {
            count = self.available.wait(count).unwrap();
        }
        *count -= 1;
        SemaphorePermit { sem: self }
    }
}

impl Drop for SemaphorePermit<'_> {
    fn drop(&mut self) {
        let mut count = self.sem.permits.lock().unwrap();
        *count += 1;
        self.sem.available.notify_one();
    }
}

fn simulate_db_query(worker_id: usize, sem: Arc<CountingSemaphore>) {
    let _permit = sem.acquire();
    let active = MAX_CONCURRENT - sem.available_permits();
    println!("worker {worker_id:>2} START  (active connections: {active}/{MAX_CONCURRENT})");

    thread::sleep(Duration::from_millis(120));

    println!("worker {worker_id:>2} FINISH");
}

fn main() {
    let sem = Arc::new(CountingSemaphore::new(MAX_CONCURRENT));
    let start = Instant::now();

    let handles: Vec<_> = (0..WORKERS)
        .map(|id| {
            let sem = Arc::clone(&sem);
            thread::spawn(move || simulate_db_query(id, sem))
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "\nall workers done in {:?}, available permits = {}",
        start.elapsed(),
        sem.available_permits()
    );
}
