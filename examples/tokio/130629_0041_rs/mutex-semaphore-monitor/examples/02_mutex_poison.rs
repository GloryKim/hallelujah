// 예제 02: Mutex poison — lock을 잡은 스레드가 panic하면 Mutex가 poisoned 됨
// 실행: cargo run --example 02_mutex_poison

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(String::from("hello")));

    let poisoner = {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut guard = data.lock().unwrap();
            guard.push_str(", world");
            panic!("intentional panic while holding the lock");
        })
    };

    let join_result = poisoner.join();
    assert!(join_result.is_err());
    println!("worker panicked: {:?}", join_result.err());

    match data.lock() {
        Ok(guard) => {
            println!("unexpected: lock succeeded -> {guard}");
        }
        Err(poisoned) => {
            let mut guard = poisoned.into_inner();
            guard.push_str(" (recovered)");
            println!("poison detected, recovered value -> {guard}");
        }
    }

    // 이후 정상 스레드도 lock 가능
    let guard = data.lock().unwrap();
    println!("final readable value -> {guard}");
}
