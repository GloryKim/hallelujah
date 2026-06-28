// 예제 05: Monitor — bounded buffer (producer / consumer)
// 실행: cargo run --example 05_monitor_bounded_buffer
//
// 고전적인 monitor 예제: 버퍼가 가득 차면 producer가 wait,
// 비어 있으면 consumer가 wait.

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

const CAPACITY: usize = 4;
const ITEMS: usize = 12;

struct BoundedBuffer<T> {
    inner: Mutex<BufferState<T>>,
    not_full: Condvar,
    not_empty: Condvar,
}

struct BufferState<T> {
    queue: VecDeque<T>,
    capacity: usize,
}

impl<T> BoundedBuffer<T> {
    fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(BufferState {
                queue: VecDeque::new(),
                capacity,
            }),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }

    fn produce(&self, item: T) {
        let mut state = self.inner.lock().unwrap();
        while state.queue.len() >= state.capacity {
            state = self.not_full.wait(state).unwrap();
        }
        state.queue.push_back(item);
        self.not_empty.notify_one();
    }

    fn consume(&self) -> T {
        let mut state = self.inner.lock().unwrap();
        while state.queue.is_empty() {
            state = self.not_empty.wait(state).unwrap();
        }
        let item = state.queue.pop_front().unwrap();
        self.not_full.notify_one();
        item
    }

    fn len(&self) -> usize {
        self.inner.lock().unwrap().queue.len()
    }
}

fn main() {
    let buffer = Arc::new(BoundedBuffer::new(CAPACITY));

    let producer = {
        let buffer = Arc::clone(&buffer);
        thread::spawn(move || {
            for item in 1..=ITEMS {
                buffer.produce(item);
                println!("[producer] +{item:>2}  (size = {}/{CAPACITY})", buffer.len());
                thread::sleep(Duration::from_millis(30));
            }
            println!("[producer] done");
        })
    };

    let consumer = {
        let buffer = Arc::clone(&buffer);
        thread::spawn(move || {
            for _ in 0..ITEMS {
                thread::sleep(Duration::from_millis(50));
                let item = buffer.consume();
                println!("[consumer] -{item:>2}  (size = {}/{CAPACITY})", buffer.len());
            }
            println!("[consumer] done");
        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();
}
