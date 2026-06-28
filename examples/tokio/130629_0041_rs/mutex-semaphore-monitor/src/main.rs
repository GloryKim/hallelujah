mod semaphore;

use semaphore::CountingSemaphore;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn demo_mutex() {
    let counter = Arc::new(Mutex::new(0_i32));
    let mut handles = Vec::new();

    for id in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..250 {
                let mut value = counter.lock().unwrap();
                *value += 1;
            }
            println!("  [mutex] thread {id} done");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  [mutex] final count = {}", *counter.lock().unwrap());
}

fn demo_semaphore() {
    let sem = Arc::new(CountingSemaphore::new(2));
    let mut handles = Vec::new();

    for id in 0..5 {
        let sem = Arc::clone(&sem);
        handles.push(thread::spawn(move || {
            let _permit = sem.acquire();
            println!(
                "  [semaphore] thread {id} acquired ({} slots left)",
                sem.available_permits()
            );
            thread::sleep(Duration::from_millis(80));
            println!("  [semaphore] thread {id} released");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn demo_monitor() {
    struct BankAccount {
        balance: Mutex<i32>,
        sufficient_funds: Condvar,
    }

    impl BankAccount {
        fn new(initial: i32) -> Self {
            Self {
                balance: Mutex::new(initial),
                sufficient_funds: Condvar::new(),
            }
        }

        fn deposit(&self, amount: i32) {
            let mut balance = self.balance.lock().unwrap();
            *balance += amount;
            println!("  [monitor] deposit {amount} -> balance {balance}");
            self.sufficient_funds.notify_one();
        }

        fn withdraw(&self, amount: i32) {
            let mut balance = self.balance.lock().unwrap();
            while *balance < amount {
                println!("  [monitor] waiting for funds (need {amount}, have {balance})");
                balance = self.sufficient_funds.wait(balance).unwrap();
            }
            *balance -= amount;
            println!("  [monitor] withdraw {amount} -> balance {balance}");
        }
    }

    let account = Arc::new(BankAccount::new(0));
    let withdrawer = {
        let account = Arc::clone(&account);
        thread::spawn(move || account.withdraw(100))
    };
    thread::sleep(Duration::from_millis(50));
    account.deposit(150);
    withdrawer.join().unwrap();
}

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Mutex · Semaphore · Monitor (Condvar)       ║");
    println!("╚══════════════════════════════════════════════╝\n");

    println!("── 1. Mutex (shared counter) ──");
    demo_mutex();

    println!("\n── 2. Semaphore (max 2 concurrent) ──");
    demo_semaphore();

    println!("\n── 3. Monitor (Mutex + Condvar) ──");
    demo_monitor();

    println!("\n자세한 예제: cargo run --example <name>");
    println!("  01_mutex_basics");
    println!("  02_mutex_poison");
    println!("  03_semaphore");
    println!("  04_monitor_condvar");
    println!("  05_monitor_bounded_buffer");
}

#[cfg(test)]
mod integration_tests {
    use super::semaphore::CountingSemaphore;
    use std::sync::{Arc, Mutex};

    #[test]
    fn mutex_counter_matches() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            handles.push(std::thread::spawn(move || {
                for _ in 0..100 {
                    *counter.lock().unwrap() += 1;
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 1000);
    }

    #[test]
    fn counting_semaphore_returns_permits() {
        let sem = CountingSemaphore::new(1);
        assert_eq!(sem.available_permits(), 1);
        let permit = sem.acquire();
        assert_eq!(sem.available_permits(), 0);
        drop(permit);
        assert_eq!(sem.available_permits(), 1);
    }
}
