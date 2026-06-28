// Counting semaphore built from Mutex + Condvar.
// (Educational stand-in when std::sync::Semaphore is unavailable.)

use std::sync::{Condvar, Mutex};

pub struct CountingSemaphore {
    permits: Mutex<usize>,
    available: Condvar,
}

pub struct SemaphorePermit<'a> {
    sem: &'a CountingSemaphore,
}

impl CountingSemaphore {
    pub fn new(permits: usize) -> Self {
        Self {
            permits: Mutex::new(permits),
            available: Condvar::new(),
        }
    }

    pub fn available_permits(&self) -> usize {
        *self.permits.lock().unwrap()
    }

    pub fn acquire(&self) -> SemaphorePermit<'_> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn limits_concurrency() {
        let sem = Arc::new(CountingSemaphore::new(2));
        let active = Arc::new(Mutex::new(0_usize));
        let peak = Arc::new(Mutex::new(0_usize));

        let handles: Vec<_> = (0..6)
            .map(|_| {
                let sem = Arc::clone(&sem);
                let active = Arc::clone(&active);
                let peak = Arc::clone(&peak);
                thread::spawn(move || {
                    let _permit = sem.acquire();
                    {
                        let mut current = active.lock().unwrap();
                        *current += 1;
                        let mut max = peak.lock().unwrap();
                        *max = (*max).max(*current);
                    }
                    thread::sleep(std::time::Duration::from_millis(20));
                    *active.lock().unwrap() -= 1;
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*peak.lock().unwrap(), 2);
        assert_eq!(sem.available_permits(), 2);
    }
}
