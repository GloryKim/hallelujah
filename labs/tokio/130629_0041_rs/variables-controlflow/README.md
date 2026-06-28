# Week 2 — Variables, Data Types & Control Flow

> **커리큘럼 위치:** Phase 1 · Foundations · Week 2  
> **목표:** Rust의 변수 선언 방식, 모든 기본 타입, 제어 흐름 완벽 이해

---

## 목차

1. [변수와 불변성 — let, let mut](#1-변수와-불변성)
2. [상수와 정적 변수 — const, static](#2-상수와-정적-변수)
3. [스칼라 타입](#3-스칼라-타입)
4. [복합 타입: 튜플, 배열, 슬라이스](#4-복합-타입)
5. [타입 추론 vs 명시적 어노테이션](#5-타입-추론)
6. [섀도잉 (Shadowing)](#6-섀도잉)
7. [if / else](#7-if--else)
8. [반복문: loop, while, for](#8-반복문)
9. [match — 패턴 매칭 기초](#9-match--패턴-매칭)
10. [예제 목록](#10-예제-목록)
11. [체크리스트](#11-체크리스트)

---

## 1. 변수와 불변성

Rust의 모든 변수는 **기본적으로 불변(immutable)**입니다. 이것은 버그 방지와 동시성 안전성을 위한 설계입니다.

```rust
let x = 5;          // 불변 변수
// x = 6;           // 오류! cannot assign twice to immutable variable

let mut y = 5;      // 가변 변수
y = 6;              // OK
```

### 왜 기본이 불변인가?

```
불변성의 이점:
  1. 의도 명확화: mut이 없으면 "이 값은 변하지 않는다"는 의도 표현
  2. 컴파일러 최적화: 불변 값은 더 공격적으로 최적화 가능
  3. 동시성 안전: 불변 값은 여러 스레드가 공유해도 안전
  4. 버그 방지: 실수로 값을 바꾸는 버그 차단
```

### let vs let mut 비교

```rust
// 불변 — 한 번 설정하면 변경 불가
let name = "Alice";
let age = 30;
let pi = 3.14159;

// 가변 — 변경 필요한 경우만
let mut counter = 0;
counter += 1;         // OK

let mut buffer = String::new();
buffer.push_str("hello");  // OK
```

---

## 2. 상수와 정적 변수

### const — 컴파일 타임 상수

```rust
// 대문자 스네이크 케이스 관례
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159265358979;
const GREETING: &str = "안녕하세요";

// const는 타입 어노테이션 필수
// const MISSING_TYPE = 42;  // 오류!

// const 표현식 — 컴파일 타임에 평가
const BUFFER_SIZE: usize = 1024 * 8;  // 8KB
const fn square(n: i32) -> i32 { n * n }
const NINE: i32 = square(3);
```

### static — 정적 변수 (프로그램 전체 수명)

```rust
// 읽기 전용 정적 변수
static LANGUAGE: &str = "Rust";
static MAX_ID: u32 = 999;

// 가변 정적 변수 — unsafe 필요 (전역 상태는 위험)
static mut COUNTER: u32 = 0;
unsafe {
    COUNTER += 1;
    println!("{}", COUNTER);
}
```

### const vs static vs let 비교

| 구분 | 저장 위치 | 수명 | 가변 여부 | 타입 어노테이션 |
|------|----------|------|----------|---------------|
| `const` | 인라인(코드에 직접 삽입) | 없음 | 불가 | 필수 |
| `static` | 데이터 세그먼트 | 프로그램 전체 | `unsafe`만 | 필수 |
| `let` | 스택(또는 힙) | 스코프 내 | `mut` 시 가능 | 생략 가능 |

---

## 3. 스칼라 타입

### 3-1. 정수 타입

```
┌─────────────────────────────────────────────────────────┐
│                   정수 타입 한눈에 보기                   │
├──────────┬─────────────┬──────────────────────────────┤
│  타입     │   크기      │   범위                        │
├──────────┼─────────────┼──────────────────────────────┤
│  i8      │  8비트      │  -128 ~ 127                  │
│  i16     │  16비트     │  -32,768 ~ 32,767            │
│  i32     │  32비트     │  ±2,147,483,647 (기본값)      │
│  i64     │  64비트     │  ±9.2 × 10¹⁸               │
│  i128    │  128비트    │  ±1.7 × 10³⁸               │
│  isize   │  아키텍처   │  포인터 크기 (32/64비트)      │
├──────────┼─────────────┼──────────────────────────────┤
│  u8      │  8비트      │  0 ~ 255                     │
│  u16     │  16비트     │  0 ~ 65,535                  │
│  u32     │  32비트     │  0 ~ 4,294,967,295           │
│  u64     │  64비트     │  0 ~ 1.8 × 10¹⁹            │
│  u128    │  128비트    │  0 ~ 3.4 × 10³⁸            │
│  usize   │  아키텍처   │  인덱싱/포인터 (배열 인덱스)  │
└──────────┴─────────────┴──────────────────────────────┘
```

**정수 오버플로 동작:**
- Debug 빌드: 오버플로 시 panic
- Release 빌드: 오버플로 시 wrap-around (조용히 오류)
- 명시적 처리: `wrapping_add`, `checked_add`, `saturating_add`, `overflowing_add`

```rust
let x: u8 = 255;

// 방어적 처리
let y = x.wrapping_add(1);     // 0 (넘침 허용)
let z = x.checked_add(1);     // None (넘치면 None)
let w = x.saturating_add(1);  // 255 (최댓값 유지)
let (v, overflow) = x.overflowing_add(1);  // (0, true)
```

### 3-2. 부동소수점

```rust
let f1: f32 = 3.14;   // 단정밀도, 약 7자리 정밀도
let f2: f64 = 3.14;   // 배정밀도, 약 15자리 정밀도 (기본값)

// 특수 값
let inf = f64::INFINITY;
let neg_inf = f64::NEG_INFINITY;
let nan = f64::NAN;

// 주의: 부동소수점 비교는 == 대신 epsilon 사용
let a = 0.1 + 0.2;
assert!((a - 0.3).abs() < f64::EPSILON);  // 안전한 비교
```

### 3-3. 불리언

```rust
let t: bool = true;
let f: bool = false;

// 불리언 연산
println!("{}", true && false);  // false (단락 평가)
println!("{}", true || false);  // true  (단락 평가)
println!("{}", !true);          // false

// if 조건에서 반드시 bool 사용 (숫자 불가!)
// if 1 { }  // 오류! C와 달리 숫자를 bool로 암시 변환 안 함
```

### 3-4. 문자 (char)

```rust
let c = 'A';              // 4바이트 유니코드 스칼라값
let emoji = '🦀';         // 이모지도 char
let korean = '한';        // 한국어도 char

// char vs &str
// char: 단일 유니코드 스칼라값 (항상 4바이트)
// &str: UTF-8 인코딩된 바이트 시퀀스 (가변 길이)
```

---

## 4. 복합 타입

### 4-1. 튜플 (Tuple)

고정 길이, **다른 타입** 가능한 복합 타입.

```rust
// 선언
let point: (f64, f64) = (3.0, 4.0);
let person: (String, u32, bool) = (String::from("Alice"), 30, true);
let unit: () = ();  // 유닛 타입 — 값이 없음을 나타냄

// 인덱스로 접근
println!("x={}, y={}", point.0, point.1);

// 구조 분해
let (name, age, employed) = person;
println!("{} is {} years old", name, age);

// 반환값으로 활용
fn min_max(v: &[i32]) -> (i32, i32) {
    let min = *v.iter().min().unwrap();
    let max = *v.iter().max().unwrap();
    (min, max)
}
let (min, max) = min_max(&[3, 1, 4, 1, 5, 9]);
```

### 4-2. 배열 (Array)

고정 길이, **같은 타입**만, **스택 할당**.

```rust
// 선언
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 100];   // 0으로 초기화된 100개짜리 배열

// 접근
println!("{}", arr[0]);   // 인덱스 접근
println!("{}", arr.len());

// 범위 초과 접근 → 런타임 panic (컴파일 타임에 알 수 없는 경우)
// let i = 10;
// arr[i];  // panic!

// 안전한 접근
if let Some(val) = arr.get(10) {
    println!("{}", val);
} else {
    println!("인덱스 범위 초과");
}

// 배열 반복
for elem in &arr {
    print!("{} ", elem);
}
println!();

for (i, elem) in arr.iter().enumerate() {
    println!("[{}] = {}", i, elem);
}
```

### 4-3. 슬라이스 (&[T], &str)

배열이나 벡터의 **일부를 참조**하는 뷰(view). 소유권 없음.

```rust
let arr = [1, 2, 3, 4, 5, 6, 7, 8];

let slice1 = &arr[2..5];  // [3, 4, 5]
let slice2 = &arr[..3];   // [1, 2, 3]
let slice3 = &arr[5..];   // [6, 7, 8]
let all = &arr[..];       // 전체

// &str 은 문자열 슬라이스
let s = String::from("hello world");
let word = &s[0..5];  // "hello"

// 함수에서 슬라이스 사용
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
println!("합: {}", sum(&arr));
println!("합: {}", sum(slice1));
```

---

## 5. 타입 추론

Rust 컴파일러는 강력한 타입 추론 엔진을 가집니다.

```rust
// 타입 추론 — 타입 생략 가능
let x = 42;             // i32로 추론
let y = 3.14;           // f64로 추론
let z = true;           // bool로 추론
let s = "hello";        // &str로 추론

// 컨텍스트 기반 추론
let v: Vec<i32> = Vec::new();  // 어노테이션 필요
let v = Vec::<i32>::new();     // 터보피쉬(::<>) 문법
let v = vec![1, 2, 3];         // 값으로부터 추론

// 클로저에서 추론
let add = |a, b| a + b;  // 첫 사용 시 타입 확정
let result = add(1, 2);  // i32로 확정됨

// 명시적 어노테이션이 필요한 경우
let large: i64 = 1_000_000_000_000;  // i32 범위 초과
let specific: u8 = 42;               // 특정 타입 필요
let parsed: i32 = "42".parse().unwrap();  // 어노테이션 없으면 모호
```

---

## 6. 섀도잉

같은 이름으로 **새로운 변수를 선언**하는 것. `mut`와 다름.

```rust
let x = 5;
let x = x + 1;     // x를 섀도잉 (x = 6)
let x = x * 2;     // 다시 섀도잉 (x = 12)
println!("{}", x); // 12

// 핵심 차이: 섀도잉은 타입도 바꿀 수 있음
let spaces = "   ";       // &str
let spaces = spaces.len(); // usize — 타입 변경!

// mut으로는 타입 변경 불가:
// let mut spaces = "   ";
// spaces = spaces.len();  // 오류! expected &str, found usize

// 실용 패턴: 파싱
let input = "42";          // &str
let input: i32 = input.parse().unwrap();  // i32로 재선언
println!("{}", input + 1); // 43
```

### 섀도잉 vs mut 선택 기준

| 상황 | 권장 |
|------|------|
| 같은 타입, 계속 변경 | `let mut` |
| 타입 변환 (파싱 등) | 섀도잉 |
| 중간 계산 후 최종값 확정 | 섀도잉 |
| 반복 조작 (카운터 등) | `let mut` |

---

## 7. if / else

```rust
// 기본 형태
let score = 85;

if score >= 90 {
    println!("A");
} else if score >= 80 {
    println!("B");
} else if score >= 70 {
    println!("C");
} else {
    println!("F");
}

// if는 표현식 — 값을 반환
let grade = if score >= 60 { "합격" } else { "불합격" };
println!("{}", grade);

// 삼항 연산자 대용
let max = if a > b { a } else { b };

// 변수 바인딩에서 (모든 분기의 타입이 일치해야 함)
let number = if true { 5 } else { 6 };  // OK
// let err = if true { 5 } else { "six" };  // 오류! 타입 불일치
```

---

## 8. 반복문

### loop — 무한 반복

```rust
// 기본 loop
let mut count = 0;
loop {
    count += 1;
    if count >= 5 {
        break;
    }
}

// loop도 표현식 — break에서 값 반환
let result = loop {
    count += 1;
    if count == 10 {
        break count * 2;  // 20 반환
    }
};
println!("{}", result);  // 20

// 중첩 루프 레이블
'outer: loop {
    loop {
        break 'outer;  // 바깥 루프 종료
    }
}
```

### while — 조건 반복

```rust
let mut n = 1;
while n < 101 {
    if n % 15 == 0 {
        println!("FizzBuzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", n);
    }
    n += 1;
}

// while let — 패턴이 맞는 동안 반복
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### for — 반복자 기반

```rust
// 범위
for i in 0..5 {    // 0,1,2,3,4
    print!("{} ", i);
}
for i in 0..=5 {   // 0,1,2,3,4,5 (끝 포함)
    print!("{} ", i);
}
for i in (0..5).rev() {  // 4,3,2,1,0 (역순)
    print!("{} ", i);
}

// 배열/벡터 반복
let fruits = ["사과", "바나나", "체리"];
for fruit in &fruits {
    println!("{}", fruit);
}

// 인덱스 포함
for (i, fruit) in fruits.iter().enumerate() {
    println!("[{}] {}", i, fruit);
}

// 가변 참조
let mut numbers = [1, 2, 3, 4, 5];
for n in &mut numbers {
    *n *= 2;
}
println!("{:?}", numbers);  // [2, 4, 6, 8, 10]

// continue — 현재 반복 건너뜀
for i in 0..10 {
    if i % 2 == 0 { continue; }
    println!("{}", i);  // 홀수만 출력
}
```

---

## 9. match — 패턴 매칭

```rust
// 기본 match
let number = 7;
match number {
    1 => println!("하나"),
    2 | 3 => println!("둘 또는 셋"),   // OR 패턴
    4..=6 => println!("넷에서 여섯"),  // 범위 패턴
    n => println!("나머지: {}", n),    // 변수 바인딩
}

// match도 표현식
let description = match number {
    1 => "one",
    2 => "two",
    _ => "other",  // 와일드카드 (나머지 모두)
};

// 튜플 match
let pair = (true, 42);
match pair {
    (true, x) if x > 0 => println!("양수: {}", x),  // guard
    (true, _) => println!("다른 케이스"),
    (false, _) => println!("false"),
}

// 구조 분해 match
struct Point { x: i32, y: i32 }
let p = Point { x: 0, y: 7 };
match p {
    Point { x: 0, y } => println!("y축 위의 점, y={}", y),
    Point { x, y: 0 } => println!("x축 위의 점, x={}", x),
    Point { x, y } => println!("({}, {})", x, y),
}

// 열거형 match (Week 8에서 자세히)
let opt: Option<i32> = Some(42);
match opt {
    Some(n) => println!("값: {}", n),
    None => println!("없음"),
}
```

---

## 10. 예제 목록

| 파일 | 주제 | 실행 방법 |
|------|------|----------|
| `examples/01_let_mut_const.rs` | let, let mut, const, static | `cargo run --example 01_let_mut_const` |
| `examples/02_scalar_types.rs` | 정수, 부동소수점, bool, char | `cargo run --example 02_scalar_types` |
| `examples/03_compound_types.rs` | 튜플, 배열, 슬라이스 | `cargo run --example 03_compound_types` |
| `examples/04_shadowing.rs` | 섀도잉 vs mut | `cargo run --example 04_shadowing` |
| `examples/05_if_else.rs` | 조건 분기, 표현식 if | `cargo run --example 05_if_else` |
| `examples/06_loops.rs` | loop, while, for, 레이블 | `cargo run --example 06_loops` |
| `examples/07_match_patterns.rs` | match 심화, 가드, 구조분해 | `cargo run --example 07_match_patterns` |
| `examples/08_fibonacci.rs` | 피보나치 (반복/재귀/이터레이터) | `cargo run --example 08_fibonacci` |
| `examples/09_temperature_converter.rs` | 온도 변환기 | `cargo run --example 09_temperature_converter` |
| `examples/10_type_inference.rs` | 타입 추론 심화 | `cargo run --example 10_type_inference` |
| `src/main.rs` | Mini-Project (숫자 맞추기 게임) | `cargo run` |

---

## 11. 체크리스트

- [ ] `let`과 `let mut`의 차이 설명 가능
- [ ] `const`와 `static`의 차이 설명 가능
- [ ] i8~i128, u8~u128 범위 대략 알고 있음
- [ ] 정수 오버플로 처리 메서드 4가지 알고 있음
- [ ] 튜플 구조 분해 사용 가능
- [ ] 배열과 슬라이스의 차이 설명 가능
- [ ] 섀도잉으로 타입 변환 해봄
- [ ] if 표현식으로 변수 할당 해봄
- [ ] `loop`에서 값 반환 해봄
- [ ] `for`로 인덱스+값 동시 순회 해봄
- [ ] `match`에서 범위 패턴과 가드 사용 가능
- [ ] 숫자 맞추기 게임 완성
