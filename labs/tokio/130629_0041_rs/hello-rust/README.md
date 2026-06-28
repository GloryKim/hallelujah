# Week 1 — Environment Setup & Hello, Rust

> **커리큘럼 위치:** Phase 1 · Foundations · Week 1  
> **목표:** Rust 개발 환경 구축, 핵심 도구 이해, 첫 프로그램 컴파일 및 실행

---

## 목차

1. [Rust란 무엇인가?](#1-rust란-무엇인가)
2. [설치 — rustup](#2-설치--rustup)
3. [툴체인 관리](#3-툴체인-관리)
4. [핵심 도구: rustc · cargo · rustfmt · clippy](#4-핵심-도구)
5. [Cargo 프로젝트 구조](#5-cargo-프로젝트-구조)
6. [Hello, World!](#6-hello-world)
7. [컴파일 모델 — LLVM 백엔드](#7-컴파일-모델--llvm-백엔드)
8. [의존성 추가 (chrono)](#8-의존성-추가-chrono)
9. [Mini-Project: 타임스탬프 인사 CLI](#9-mini-project-타임스탬프-인사-cli)
10. [예제 목록](#10-예제-목록)
11. [체크리스트](#11-체크리스트)

---

## 1. Rust란 무엇인가?

Rust는 2015년 Mozilla가 발표한 **시스템 프로그래밍 언어**입니다.

### 핵심 설계 목표

| 목표 | 설명 |
|------|------|
| **안전성** | 메모리 안전 보장 (null 포인터, dangling 포인터, 데이터 레이스 없음) |
| **성능** | C/C++에 필적하는 실행 속도 (Zero-cost abstraction) |
| **동시성** | 컴파일 타임에 데이터 레이스를 차단하는 "Fearless Concurrency" |

### 다른 언어와의 비교

```
┌─────────────────────────────────────────────────────────┐
│              메모리 관리 방식 비교                        │
├──────────────┬──────────────┬─────────────────────────┤
│   언어        │   방식       │   특징                   │
├──────────────┼──────────────┼─────────────────────────┤
│ C / C++      │ 수동 관리    │ 빠르지만 취약            │
│ Java / Go    │ GC           │ 안전하지만 런타임 오버헤드 │
│ Rust         │ 소유권 시스템 │ 안전 + 빠름 (GC 없음)   │
└──────────────┴──────────────┴─────────────────────────┘
```

Rust의 핵심 혁신은 **Ownership(소유권)** 시스템입니다. 런타임 가비지 컬렉터 없이, 컴파일러가 메모리 안전을 **컴파일 타임**에 검증합니다.

---

## 2. 설치 — rustup

`rustup`은 Rust 공식 툴체인 인스톨러 겸 버전 관리자입니다.

### 설치

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows: https://rustup.rs 에서 rustup-init.exe 다운로드
```

### 설치 확인

```bash
rustc --version      # 컴파일러 버전
cargo --version      # 빌드 도구 버전
rustup --version     # 툴체인 관리자 버전
```

예상 출력:
```
rustc 1.95.0 (2026-xx-xx)
cargo 1.95.0 (2026-xx-xx)
rustup 1.27.0 (2026-xx-xx)
```

---

## 3. 툴체인 관리

```bash
# 기본 stable 채널 사용 (권장)
rustup default stable

# 특정 버전으로 전환
rustup default 1.95.0

# nightly 채널 설치 (실험적 기능)
rustup install nightly
rustup default nightly

# 업데이트
rustup update

# 설치된 툴체인 목록
rustup toolchain list

# 타겟 추가 (예: WebAssembly)
rustup target add wasm32-unknown-unknown

# 타겟 목록 확인
rustup target list --installed
```

### 채널 비교

| 채널 | 설명 | 권장 용도 |
|------|------|----------|
| `stable` | 안정 릴리스 (6주 주기) | 프로덕션, 학습 |
| `beta` | 다음 stable의 후보 | 사전 호환성 테스트 |
| `nightly` | 매일 빌드, 실험적 기능 포함 | 최신 기능 사용 |

---

## 4. 핵심 도구

### 4-1. rustc — 컴파일러

`rustc`는 Rust 소스코드를 컴파일하는 핵심 도구입니다.

```bash
# 단일 파일 직접 컴파일 (학습/실험용)
rustc main.rs         # 현재 디렉토리에 실행 파일 생성
rustc main.rs -o myapp  # 출력 이름 지정

# 최적화 빌드
rustc -O main.rs

# MIR(중간 표현) 출력 (내부 동작 학습용)
rustc --emit mir main.rs
```

> 실무에서는 `cargo`가 `rustc`를 대신 호출합니다. 직접 `rustc`를 쓰는 경우는 드뭅니다.

### 4-2. cargo — 빌드 시스템 & 패키지 관리자

Cargo는 Rust의 공식 빌드 도구입니다. npm(Node.js), pip(Python)과 유사하지만 더 강력합니다.

```bash
# 프로젝트 생성
cargo new my_project          # 바이너리 프로젝트
cargo new my_lib --lib        # 라이브러리 프로젝트

# 빌드
cargo build                   # 디버그 빌드 (target/debug/)
cargo build --release         # 릴리스 빌드 (target/release/) - 최적화 포함

# 실행
cargo run                     # 빌드 후 즉시 실행
cargo run --release           # 릴리스 빌드 후 실행
cargo run --example 01_hello  # 특정 예제 실행

# 테스트
cargo test                    # 모든 테스트 실행

# 의존성 설치
cargo add chrono              # 최신 버전 자동 추가
cargo add rand@0.8            # 특정 버전 지정

# 의존성 업데이트
cargo update

# 문서 생성 및 브라우저에서 열기
cargo doc --open

# 검사만 (컴파일 없이 오류 확인 — 빠름)
cargo check

# 벤치마크
cargo bench
```

### 4-3. rustfmt — 코드 포매터

```bash
# 현재 프로젝트 전체 포맷
cargo fmt

# 단일 파일 포맷
rustfmt src/main.rs

# 포맷 확인만 (변경 없이)
cargo fmt -- --check
```

`rustfmt.toml`로 스타일 설정:
```toml
edition = "2021"
max_width = 100
tab_spaces = 4
use_small_heuristics = "Default"
```

### 4-4. clippy — 린터

Clippy는 Rust 코드의 일반적인 실수, 비관용적 패턴, 성능 문제를 찾아주는 정적 분석 도구입니다.

```bash
# 린트 실행
cargo clippy

# 경고를 오류로 처리 (CI에서 유용)
cargo clippy -- -D warnings

# 특정 경고 허용
cargo clippy -- -A clippy::needless_return
```

clippy가 잡아주는 예시:
```rust
// 나쁜 코드 — clippy가 경고
let v = vec![1, 2, 3];
if v.len() == 0 {  // clippy: use `is_empty()` instead
    println!("empty");
}

// 좋은 코드
if v.is_empty() {
    println!("empty");
}
```

---

## 5. Cargo 프로젝트 구조

```
my_project/
├── Cargo.toml          ← 프로젝트 메타데이터 + 의존성 목록
├── Cargo.lock          ← 의존성 정확한 버전 잠금 (자동 생성)
├── src/
│   ├── main.rs         ← 바이너리 진입점 (fn main)
│   └── lib.rs          ← 라이브러리 진입점 (있을 때)
├── examples/
│   └── demo.rs         ← cargo run --example demo 로 실행
├── tests/
│   └── integration.rs  ← 통합 테스트
├── benches/
│   └── bench.rs        ← 벤치마크
└── target/             ← 빌드 결과물 (gitignore에 추가)
    ├── debug/
    └── release/
```

### Cargo.toml 구조

```toml
[package]
name = "my_project"      # 프로젝트 이름 (소문자, 하이픈 사용)
version = "0.1.0"        # SemVer (major.minor.patch)
edition = "2021"         # Rust 에디션 (2015, 2018, 2021)
authors = ["이름 <email>"]
description = "프로젝트 설명"
license = "MIT"

[dependencies]
# 버전 지정 방법
chrono = "0.4"           # ^0.4 (0.4.x 중 최신)
rand = "=0.8.5"          # 정확히 이 버전
serde = { version = "1", features = ["derive"] }

[dev-dependencies]       # 테스트/벤치마크에서만 사용
criterion = "0.5"

[profile.release]        # 릴리스 빌드 최적화 설정
opt-level = 3
lto = true
```

### Cargo.lock

- **바이너리 프로젝트**: Cargo.lock을 git에 커밋 (재현 가능한 빌드)
- **라이브러리 크레이트**: Cargo.lock은 gitignore (사용자가 버전 선택)

---

## 6. Hello, World!

### 가장 기본적인 Rust 프로그램

```rust
fn main() {
    println!("Hello, World!");
}
```

`println!`은 함수가 아니라 **매크로**입니다. `!`가 붙은 것이 매크로의 표시입니다.

### println! 포맷 지정자

```rust
fn main() {
    let name = "Rust";
    let version = 1.95;
    let count = 42usize;

    // 기본 출력
    println!("Hello, {}!", name);

    // 디버그 출력
    println!("{:?}", (name, version));      // ("Rust", 1.95)
    println!("{:#?}", (name, version));     // 줄바꿈 포함 pretty-print

    // 숫자 포맷
    println!("{:5}", count);       // "   42" (우측 정렬, 폭 5)
    println!("{:<5}", count);      // "42   " (좌측 정렬)
    println!("{:05}", count);      // "00042" (0으로 채움)
    println!("{:b}", count);       // "101010" (2진수)
    println!("{:x}", count);       // "2a" (16진수 소문자)
    println!("{:X}", count);       // "2A" (16진수 대문자)
    println!("{:o}", count);       // "52" (8진수)
    println!("{:e}", version);     // "1.95e0" (과학 표기)

    // 소수점
    println!("{:.2}", version);    // "1.95" (소수점 2자리)
    println!("{:8.3}", version);   // "   1.950" (폭 8, 소수점 3자리)

    // 다중 인자 (위치 지정)
    println!("{0}은 {1}, {1}은 {0}", "닭", "달걀");
}
```

### 다른 출력 매크로들

```rust
// 표준 출력 (개행 없음)
print!("줄바꿈 없음");

// 표준 에러 출력
eprintln!("오류: {}", "문제 발생");
eprint!("에러 없는 줄바꿈");

// 포맷 문자열을 String으로 반환
let s = format!("안녕, {}!", "세계");

// 디버그/오류 시 즉시 종료
// panic!("치명적 오류: {}", "원인");
```

---

## 7. 컴파일 모델 — LLVM 백엔드

```
Rust 소스코드 (.rs)
        │
        ▼
   [rustc 프론트엔드]
        │ 렉싱 + 파싱
        ▼
      AST (추상 구문 트리)
        │ HIR 변환 (High-level IR)
        ▼
      HIR (타입 검사, 트레이트 해석)
        │ MIR 변환 (Mid-level IR)
        ▼
      MIR (소유권/차용 검사, 최적화)
        │ LLVM IR 생성
        ▼
    LLVM IR
        │ LLVM 최적화 패스
        ▼
  기계어 코드 (x86, ARM, WASM 등)
```

### 단계별 역할

| 단계 | 담당 | 주요 작업 |
|------|------|----------|
| **AST** | rustc | 문법 파싱, 매크로 확장 |
| **HIR** | rustc | 타입 추론, 트레이트 해석 |
| **MIR** | rustc | **소유권/차용 검사**, 상수 평가 |
| **LLVM IR** | LLVM | 플랫폼 독립 최적화 |
| **기계어** | LLVM | 타겟별 코드 생성 |

```bash
# 각 단계 출력 확인
rustc --emit=ast main.rs       # AST
rustc --emit=hir main.rs       # HIR  
rustc --emit=mir main.rs       # MIR (가장 교육적)
rustc --emit=llvm-ir main.rs   # LLVM IR
rustc --emit=asm main.rs       # 어셈블리
```

---

## 8. 의존성 추가 (chrono)

### Cargo.toml에 추가

```toml
[dependencies]
chrono = "0.4"
```

또는 CLI로:
```bash
cargo add chrono
```

### chrono 사용 예시

```rust
use chrono::{Local, Utc, DateTime, Duration};

fn main() {
    // 로컬 현재 시간
    let now_local = Local::now();
    println!("로컬 시간: {}", now_local);

    // UTC 시간
    let now_utc = Utc::now();
    println!("UTC 시간: {}", now_utc);

    // 포맷 지정
    println!("{}", now_local.format("%Y년 %m월 %d일 %H:%M:%S"));
    println!("{}", now_local.format("%Y-%m-%d"));
    println!("{}", now_local.format("%A, %B %d, %Y"));  // 영어 요일/월

    // 날짜 연산
    let tomorrow = now_local + Duration::days(1);
    println!("내일: {}", tomorrow.format("%Y-%m-%d"));

    let last_week = now_local - Duration::weeks(1);
    println!("지난주: {}", last_week.format("%Y-%m-%d"));
}
```

---

## 9. Mini-Project: 타임스탬프 인사 CLI

> **목표:** 이름을 받아 현재 시각과 함께 인사 메시지를 출력하는 CLI

`src/main.rs` 참조 — 완전한 구현 포함.

### 기능 목록

- [x] 명령행 인자로 이름 입력 받기
- [x] 인자가 없으면 환경 변수에서 읽기
- [x] 모두 없으면 "세계" 기본값
- [x] 현재 날짜+시간 포맷팅
- [x] 시간대별 인사 (아침/오후/저녁/밤)
- [x] 스타일 있는 출력

---

## 10. 예제 목록

| 파일 | 주제 | 실행 방법 |
|------|------|----------|
| `examples/01_hello_world.rs` | 기본 Hello World + println! 포맷 | `cargo run --example 01_hello_world` |
| `examples/02_print_variants.rs` | print!, eprintln!, format! 변형들 | `cargo run --example 02_print_variants` |
| `examples/03_data_types_intro.rs` | 기본 타입 미리보기 | `cargo run --example 03_data_types_intro` |
| `examples/04_cargo_tools_demo.rs` | 도구 사용법 안내 코드 | `cargo run --example 04_cargo_tools_demo` |
| `examples/05_chrono_timestamp.rs` | chrono로 시간 다루기 | `cargo run --example 05_chrono_timestamp` |
| `examples/06_compilation_model.rs` | 컴파일 단계 탐색 | `cargo run --example 06_compilation_model` |
| `src/main.rs` | Mini-Project (타임스탬프 인사 CLI) | `cargo run` |

---

## 11. 체크리스트

- [ ] `rustup`, `rustc`, `cargo` 설치 완료
- [ ] `cargo new`로 프로젝트 생성
- [ ] `src/main.rs` 수정 후 `cargo run` 성공
- [ ] `cargo fmt` 실행 — 포맷 변경 없음 확인
- [ ] `cargo clippy` 실행 — 경고 없음 확인
- [ ] `chrono` 의존성 추가 후 현재 시간 출력 성공
- [ ] Mini-Project 완성 및 실행 확인

---

## 참고 자료

- [The Rust Book — Chapter 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [cargo 공식 문서](https://doc.rust-lang.org/cargo/)
- [chrono 크레이트 문서](https://docs.rs/chrono)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
