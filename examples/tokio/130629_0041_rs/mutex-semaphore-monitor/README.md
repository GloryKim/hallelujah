# Mutex · Semaphore · Monitor

> **커리큘럼 위치:** Rust syntax · concurrency primitives  
> **목표:** `Mutex`, `Semaphore`, Monitor 패턴(`Mutex` + `Condvar`)의 역할과 사용법 이해

---

## 개념 정리

| 개념 | Rust API | 역할 |
| :--- | :--- | :--- |
| **Mutex** | `std::sync::Mutex` | 한 번에 하나의 스레드만 공유 데이터 접근 |
| **Semaphore** | `Mutex<usize>` + `Condvar` | 동시에 N개까지 접근 허용 (counting semaphore) |
| **Monitor** | `Mutex` + `Condvar` | lock 안에서 조건을 기다리고(`wait`), 변경 시 깨우기(`notify`) |

Rust에는 Java/C#의 `monitor` 키워드가 없다. **Monitor 패턴**은 `Mutex`로 상태를 보호하고 `Condvar`로 조건 동기화를 구현한다.

---

## 실행 방법

```bash
# 개요 (세 가지 패턴을 짧게 시연)
cargo run

# 개별 예제
cargo run --example 01_mutex_basics
cargo run --example 02_mutex_poison
cargo run --example 03_semaphore
cargo run --example 04_monitor_condvar
cargo run --example 05_monitor_bounded_buffer
```

---

## 예제 목록

| # | 파일 | 내용 |
| :-: | :--- | :--- |
| 01 | `01_mutex_basics` | `Arc<Mutex<T>>` + 멀티 스레드 카운터 |
| 02 | `02_mutex_poison` | lock 보유 중 panic → poison 복구 |
| 03 | `03_semaphore` | 연결 풀 시뮬레이션 (동시 3개 제한) |
| 04 | `04_monitor_condvar` | 송수신 박스 — wait / notify |
| 05 | `05_monitor_bounded_buffer` | producer / consumer bounded buffer |

---

## 1. Mutex

```rust
use std::sync::{Arc, Mutex};

let data = Arc::new(Mutex::new(0));
let mut guard = data.lock().unwrap();  // LockResult<T>
*guard += 1;
// guard가 drop되면 lock 해제
```

- `lock()`은 `LockResult<MutexGuard<T>>`를 반환한다.
- guard는 `Deref` / `DerefMut`으로 내부 값에 접근한다.
- 스레드 간 공유는 `Arc<Mutex<T>>` 조합이 일반적이다.

---

## 2. Semaphore

Rust 표준 라이브러리의 counting semaphore 개념을 **Mutex + Condvar** 로 구현한다.
(세마포어 자체가 monitor 패턴의 한 형태다.)

```rust
struct CountingSemaphore {
    permits: Mutex<usize>,
    available: Condvar,
}

let sem = CountingSemaphore::new(3);   // permit 3개
let permit = sem.acquire();            // permit 1개 획득 (0이면 wait)
drop(permit);                          // permit 반환 + notify_one
```

- **Binary semaphore** (0/1) ≈ Mutex와 유사하지만, permit 소유권을 다른 스레드로 넘기기 쉽다.
- **Counting semaphore** — DB 연결 풀, 동시 요청 제한 등에 사용한다.

---

## 3. Monitor (Mutex + Condvar)

```rust
use std::sync::{Condvar, Mutex};

let pair = Arc::new((Mutex::new(false), Condvar::new()));
let (lock, cvar) = &*pair;

let mut started = lock.lock().unwrap();
while !*started {
    started = cvar.wait(started).unwrap();  // lock 해제 후 대기, 깨어나면 lock 재획득
}
```

- `wait(guard)` : mutex를 **원자적으로** 풀고 sleep → 깨어나면 다시 lock
- `notify_one()` / `notify_all()` : 대기 중인 스레드 깨우기
- **항상 `while` 루프**로 조건을 검사한다 (spurious wakeup 대비)

---

## 체크리스트

- [ ] `Mutex::lock()`이 반환하는 guard의 수명과 lock 해제 시점을 설명할 수 있다
- [ ] poisoned mutex를 `into_inner()`로 복구할 수 있다
- [ ] Semaphore permit 수와 `available_permits()` 의미를 안다
- [ ] Monitor에서 `wait` 전후로 mutex가 어떻게 동작하는지 설명할 수 있다
- [ ] bounded buffer에서 `not_full` / `not_empty` Condvar 역할을 구분할 수 있다

---

## Changelog

- 260618_0001 : Mutex, Semaphore, Monitor(Condvar) 예제 추가
