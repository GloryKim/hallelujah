// 예제 04: Monitor 패턴 — Mutex + Condvar 로 조건 대기/알림
// 실행: cargo run --example 04_monitor_condvar
//
// Java의 synchronized + wait/notify 와 유사한 역할:
//   - Mutex  : 공유 상태를 감싸는 lock
//   - Condvar: "조건이 만족될 때까지" wait, 조건 변경 시 notify

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct MessageBox {
    message: Mutex<Option<String>>,
    ready: Condvar,
}

impl MessageBox {
    fn new() -> Self {
        Self {
            message: Mutex::new(None),
            ready: Condvar::new(),
        }
    }

    fn send(&self, text: String) {
        let mut guard = self.message.lock().unwrap();
        *guard = Some(text.clone());
        println!("[sender] stored: {text}");
        self.ready.notify_one();
    }

    fn receive(&self) -> String {
        let mut guard = self.message.lock().unwrap();
        while guard.is_none() {
            println!("[receiver] waiting for message...");
            guard = self.ready.wait(guard).unwrap();
        }
        guard.take().unwrap()
    }
}

fn main() {
    let box_ = Arc::new(MessageBox::new());

    let receiver = {
        let box_ = Arc::clone(&box_);
        thread::spawn(move || {
            let msg = box_.receive();
            println!("[receiver] got: {msg}");
        })
    };

    thread::sleep(Duration::from_millis(200));
    box_.send(String::from("hello from monitor"));

    receiver.join().unwrap();
}
