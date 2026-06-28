// 예제 01: Mutex 기초 — 여러 스레드가 하나의 값을 안전하게 수정
// 실행: cargo run --example 01_mutex_basics

use std::sync::{Arc, Mutex};
use std::thread;

fn increment_with_mutex(values: Arc<Mutex<Vec<i32>>>, thread_id: usize, rounds: usize) {
    for round in 0..rounds {
        let mut guard = values.lock().unwrap();
        let last = guard.last().copied().unwrap_or(0);
        guard.push(last + 1);
        if round % 100 == 0 {
            println!("thread {thread_id}: len = {}", guard.len());
        }
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(vec![0]));

    let handles: Vec<_> = (0..3)
        .map(|id| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || increment_with_mutex(shared, id, 300))
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let guard = shared.lock().unwrap();
    println!("\nfinal length : {}", guard.len());
    println!("final value  : {}", guard.last().unwrap());
    println!("expected len : {}", 1 + 3 * 300);
}
