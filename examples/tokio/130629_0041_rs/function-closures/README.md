# Week 3 — Functions, Closures & Iterators

> **커리큘럼 위치:** Phase 1 · Foundations · Week 3  
> **목표:** 함수 문법, 클로저의 세 가지 트레이트, 이터레이터 어댑터 체인 완전 이해

---

## 목차

1. [함수 문법](#1-함수-문법)
2. [표현식 vs 문장 (Expressions vs Statements)](#2-표현식-vs-문장)
3. [클로저 기초](#3-클로저-기초)
4. [Fn / FnMut / FnOnce 트레이트](#4-fn--fnmut--fnonce-트레이트)
5. [move 클로저](#5-move-클로저)
6. [이터레이터 기초](#6-이터레이터-기초)
7. [이터레이터 어댑터](#7-이터레이터-어댑터)
8. [고차 함수 패턴](#8-고차-함수-패턴)
9. [예제 목록](#9-예제-목록)
10. [체크리스트](#10-체크리스트)

---

## 1. 함수 문법

### 기본 구조

```rust
fn 함수이름(파라미터: 타입, ...) -> 반환타입 {
    // 본문
    반환값  // 세미콜론 없음
}
```

```rust
// 반환값 없음 — 암시적으로 () 반환
fn greet(name: &str) {
    println!("안녕, {}!", name);
}

// 반환값 있음
fn add(a: i32, b: i32) -> i32 {
    a + b  // 세미콜론 없음 = 표현식 = 반환
}

// 여러 값 반환 (튜플 활용)
fn min_max(v: &[i32]) -> (i32, i32) {
    (*v.iter().min().unwrap(), *v.iter().max().unwrap())
}

// 일찍 반환 (guard clause 패턴)
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;  // 명시적 return
    }
    Some(a / b)       // 마지막 표현식
}
```

### 파라미터 패턴

```rust
// 구조 분해
fn print_point(&(x, y): &(i32, i32)) {
    println!("({}, {})", x, y);
}

// 슬라이스 파라미터 (길이 무관)
fn first_two(s: &[i32]) -> Option<(i32, i32)> {
    match s {
        [a, b, ..] => Some((*a, *b)),
        _ => None,
    }
}

// 제네릭 파라미터 (Week 13에서 자세히)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max { max = item; }
    }
    max
}
```

### 함수 포인터

```rust
// 함수도 타입을 가짐: fn(i32, i32) -> i32
fn apply(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}

fn multiply(a: i32, b: i32) -> i32 { a * b }

let result = apply(multiply, 3, 4);  // 12
let result2 = apply(|a, b| a + b, 3, 4);  // 7 (클로저도 가능)
```

---

## 2. 표현식 vs 문장

Rust에서 거의 모든 것은 **표현식(expression)** — 값을 반환합니다.

```
표현식: 값을 생성  ← 세미콜론 없음
문장:   효과만 있음 ← 세미콜론 있음
```

```rust
// 문장 (statement) — ; 로 끝남, 값 없음
let x = 5;         // 선언문
println!("hi");    // 매크로 문

// 표현식 (expression) — ; 없음, 값 있음
5 + 3              // 정수 8
"hello"            // &str
{ let y = 3; y + 1 }  // 블록 표현식 = 4

// 함수 반환값:
fn five() -> i32 {
    5              // 표현식 — 반환됨
}

fn five_stmt() -> i32 {
    5;             // 문장 — () 반환! 컴파일 오류
    // ^ 마지막 문장이 () 이므로 i32 반환 타입과 불일치
}

// if 표현식
let y = if x > 5 { 10 } else { 20 };

// 블록 표현식
let z = {
    let base = 10;
    let power = 3;
    (0..power).fold(1i32, |acc, _| acc * base)  // 1000
};
```

---

## 3. 클로저 기초

클로저는 **주변 환경을 캡처할 수 있는 익명 함수**입니다.

### 문법

```
|파라미터| 표현식

|파라미터: 타입| -> 반환타입 { 본문 }
```

```rust
// 다양한 클로저 문법
let add = |a, b| a + b;           // 가장 간결
let add2 = |a: i32, b: i32| a + b;
let add3 = |a: i32, b: i32| -> i32 { a + b };

// 환경 캡처
let offset = 10;
let add_offset = |x| x + offset;  // offset 캡처 (참조로)
println!("{}", add_offset(5));     // 15

// 여전히 offset 사용 가능 (캡처는 참조)
println!("offset: {}", offset);

// 가변 캡처
let mut count = 0;
let mut increment = || {
    count += 1;
    count
};
println!("{}", increment());  // 1
println!("{}", increment());  // 2
println!("{}", increment());  // 3
// println!("{}", count);  // 오류! increment가 &mut 빌림 중
drop(increment);
println!("{}", count);        // 3 (OK — increment 드롭됨)
```

### 캡처 모드

```
┌─────────────────────────────────────────────────────┐
│              클로저 캡처 모드                         │
├─────────────────┬──────────────────────────────────┤
│  캡처 방식       │  설명                             │
├─────────────────┼──────────────────────────────────┤
│  공유 참조 (&T)  │  읽기만 필요할 때 (자동 선택)     │
│  가변 참조 (&mut)│  수정 필요할 때 (자동 선택)       │
│  소유권 이동     │  move 키워드로 강제               │
└─────────────────┴──────────────────────────────────┘
```

---

## 4. Fn / FnMut / FnOnce 트레이트

클로저는 자동으로 세 가지 트레이트 중 하나(또는 여럿)를 구현합니다.

### 트레이트 계층

```
FnOnce (모든 클로저)
  └─ FnMut (환경을 수정하지 않거나 수정하는 클로저)
        └─ Fn (환경을 수정하지 않는 클로저)
```

```rust
// FnOnce — 한 번만 호출 가능 (값을 이동시킴)
fn consume(f: impl FnOnce() -> String) {
    println!("{}", f());
    // f();  // 오류! 이미 소비됨
}
let s = String::from("hello");
consume(move || s);  // s를 이동시키므로 FnOnce

// FnMut — 여러 번 호출 가능, 환경 수정 가능
fn call_three_times(mut f: impl FnMut()) {
    f(); f(); f();
}
let mut count = 0;
call_three_times(|| count += 1);
// count == 3

// Fn — 여러 번 호출 가능, 환경 수정 불가
fn call_twice(f: impl Fn(i32) -> i32) -> (i32, i32) {
    (f(1), f(2))
}
let multiplier = 10;
let result = call_twice(|x| x * multiplier);
// result == (10, 20)
```

### 언제 무엇을 사용하나?

| 상황 | 사용 트레이트 |
|------|-------------|
| 값을 이동/소비하는 클로저 | `FnOnce` |
| 내부 상태를 수정하는 클로저 | `FnMut` |
| 순수 함수처럼 동작하는 클로저 | `Fn` |
| 가장 유연하게 받으려면 | `FnOnce` (가장 느슨한 요구) |
| 여러 번 호출해야 하면 | `Fn` 또는 `FnMut` |

---

## 5. move 클로저

```rust
// 문제: 캡처 참조가 원본보다 오래 살아남을 수 없음
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    // x는 함수 스택 프레임에 있음
    // move 없으면: x의 참조가 반환 후 dangling!
    move |y| x + y  // move: x를 클로저로 이동
}
let add5 = make_adder(5);
println!("{}", add5(10));  // 15

// 스레드에서 필수
let data = vec![1, 2, 3];
std::thread::spawn(move || {
    println!("{:?}", data);  // data가 스레드로 이동됨
}).join().unwrap();
// println!("{:?}", data);  // 오류! data가 이동됨
```

---

## 6. 이터레이터 기초

### 세 가지 반복 메서드

```rust
let v = vec![1, 2, 3];

// iter() — 공유 참조 반환 (&T)
for x in v.iter() {
    println!("{}", x);  // x: &i32
}
println!("{:?}", v);  // v는 여전히 유효

// iter_mut() — 가변 참조 반환 (&mut T)
let mut v2 = vec![1, 2, 3];
for x in v2.iter_mut() {
    *x *= 2;           // x: &mut i32
}
println!("{:?}", v2);  // [2, 4, 6]

// into_iter() — 소유권 이동 (T)
let v3 = vec!["a", "b", "c"];
for s in v3.into_iter() {
    println!("{}", s);  // s: &str (String이었다면 String)
}
// println!("{:?}", v3);  // 오류! v3이 이동됨
```

### Iterator 트레이트

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 수백 개의 기본 메서드들...
}
```

---

## 7. 이터레이터 어댑터

어댑터는 이터레이터를 변환해 새 이터레이터를 반환합니다. **지연 평가(lazy)**.

```rust
let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// map — 각 요소를 변환
let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();

// filter — 조건에 맞는 요소만
let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();

// filter_map — filter + map (Option 반환)
let strings = vec!["1", "two", "3", "four", "5"];
let numbers: Vec<i32> = strings.iter()
    .filter_map(|s| s.parse().ok())
    .collect();  // [1, 3, 5]

// flat_map — map + flatten
let words = vec!["hello world", "foo bar"];
let chars: Vec<&str> = words.iter()
    .flat_map(|s| s.split_whitespace())
    .collect();  // ["hello", "world", "foo", "bar"]

// take / skip
let first3: Vec<&i32> = v.iter().take(3).collect();
let after3: Vec<&i32> = v.iter().skip(3).collect();

// enumerate — (인덱스, 값) 쌍
for (i, val) in v.iter().enumerate() {
    println!("[{}] = {}", i, val);
}

// zip — 두 이터레이터 합치기
let names = vec!["Alice", "Bob"];
let scores = vec![95, 87];
let paired: Vec<_> = names.iter().zip(scores.iter()).collect();

// chain — 이어 붙이기
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];
let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();

// fold — 누적 연산 (= reduce + 초기값)
let sum = v.iter().fold(0, |acc, &x| acc + x);
let product = v.iter().fold(1, |acc, &x| acc * x);

// reduce — fold의 초기값 없는 버전
let max = v.iter().copied().reduce(|a, b| if a > b { a } else { b });

// collect — 소비하면서 컬렉션 생성
let set: std::collections::HashSet<i32> = v.iter().copied().collect();
let map: std::collections::HashMap<usize, i32> =
    v.iter().copied().enumerate().collect();
```

---

## 8. 고차 함수 패턴

```rust
// 함수를 인자로 받음
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}
let result = apply_twice(|x| x + 3, 7);  // 13

// 함수를 반환
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}
let triple = make_multiplier(3);
let result = triple(7);  // 21

// 함수 합성
fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}
let add1 = |x: i32| x + 1;
let double = |x: i32| x * 2;
let add1_then_double = compose(add1, double);
println!("{}", add1_then_double(5));  // (5+1)*2 = 12
```

---

## 9. 예제 목록

| 파일 | 주제 | 실행 방법 |
|------|------|----------|
| `examples/01_functions_basics.rs` | 함수 선언, 파라미터, 반환 | `cargo run --example 01_functions_basics` |
| `examples/02_expressions_statements.rs` | 표현식 vs 문장 | `cargo run --example 02_expressions_statements` |
| `examples/03_closures_basics.rs` | 클로저 문법, 캡처 모드 | `cargo run --example 03_closures_basics` |
| `examples/04_fn_traits.rs` | Fn / FnMut / FnOnce | `cargo run --example 04_fn_traits` |
| `examples/05_move_closures.rs` | move 클로저, 수명 | `cargo run --example 05_move_closures` |
| `examples/06_iterators_basics.rs` | iter/iter_mut/into_iter | `cargo run --example 06_iterators_basics` |
| `examples/07_iterator_adapters.rs` | map, filter, fold, ... | `cargo run --example 07_iterator_adapters` |
| `examples/08_higher_order_functions.rs` | 고차 함수, 함수 합성 | `cargo run --example 08_higher_order_functions` |
| `examples/09_factorial.rs` | 팩토리얼 다양한 구현 | `cargo run --example 09_factorial` |
| `examples/10_fold_and_scan.rs` | fold, scan, reduce | `cargo run --example 10_fold_and_scan` |
| `src/main.rs` | Mini-Project (단어 빈도 분석기) | `cargo run` |

---

## 10. 체크리스트

- [ ] 함수 파라미터와 반환 타입 선언 가능
- [ ] 마지막 표현식이 반환값임을 이해
- [ ] 클로저를 변수에 저장하고 호출 가능
- [ ] 클로저가 환경을 캡처하는 것 이해
- [ ] `Fn`, `FnMut`, `FnOnce` 차이 설명 가능
- [ ] `move` 클로저가 필요한 상황 이해
- [ ] `iter()`, `iter_mut()`, `into_iter()` 차이 이해
- [ ] `map`, `filter`, `fold`, `collect` 사용 가능
- [ ] 이터레이터 어댑터 체인 3개 이상 연결 가능
- [ ] 고차 함수 작성 가능 (클로저를 인자/반환값으로)
- [ ] 단어 빈도 분석기 완성
