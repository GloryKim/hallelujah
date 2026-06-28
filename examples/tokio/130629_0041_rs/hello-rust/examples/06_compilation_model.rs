// 예제 06: 컴파일 모델 탐구 + 인라인 어셈블리 힌트
// 실행: cargo run --example 06_compilation_model
// MIR 출력: rustc --emit mir examples/06_compilation_model.rs

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

// const fn — 컴파일 타임에 평가 가능 (Rust 1.95에서 많은 표준 함수가 const)
const fn const_add(a: i32, b: i32) -> i32 {
    a + b
}

const MAX_SIZE: i32 = const_add(100, 24);  // 컴파일 타임에 124로 확정

// 인라인 함수 힌트
#[inline(always)]
fn hot_path(x: i32) -> i32 {
    x * x + 2 * x + 1
}

#[inline(never)]
fn cold_path(x: i32) -> i32 {
    x.pow(3)
}

fn demonstrate_zero_cost() {
    // 제네릭 → 모노모피제이션 → 제로 코스트 추상화
    fn identity<T>(x: T) -> T {
        x  // 컴파일러가 T를 각 타입으로 전개 (단형화)
    }

    let a = identity(42_i32);
    let b = identity(3.14_f64);
    let c = identity("hello");

    println!("identity(42): {}", a);
    println!("identity(3.14): {}", b);
    println!("identity(\"hello\"): {}", c);
    // 위 세 호출은 런타임 오버헤드 없이 완전히 인라인/특수화됨
}

fn main() {
    println!("=== Rust 컴파일 모델 탐구 ===\n");

    println!("── 컴파일 파이프라인 ──");
    println!(
        r#"
  ┌─────────────────────────────────────────────────────┐
  │                 rustc 컴파일 파이프라인              │
  ├─────────────────────────────────────────────────────┤
  │  .rs 소스코드                                        │
  │      ↓ 렉싱 + 파싱                                  │
  │  AST (추상 구문 트리)                                │
  │      ↓ 매크로 확장                                  │
  │  AST (확장됨)                                        │
  │      ↓ 이름 해석 + 타입 체크                        │
  │  HIR (High-level IR)                                 │
  │      ↓ 차용 검사 (Borrow Checker)                   │
  │  MIR (Mid-level IR)  ← 가장 중요                    │
  │      ↓ 최적화 (인라인, 상수 폴딩 등)                │
  │  MIR (최적화됨)                                      │
  │      ↓ LLVM IR 생성                                 │
  │  LLVM IR                                             │
  │      ↓ LLVM 최적화 패스                             │
  │  기계어 (.o 오브젝트 파일)                           │
  │      ↓ 링킹                                          │
  │  실행 파일                                           │
  └─────────────────────────────────────────────────────┘
"#
    );

    println!("── 상수 평가 ──");
    println!("MAX_SIZE = {} (컴파일 타임 계산)", MAX_SIZE);
    const ARRAY_SIZE: usize = 5;
    let arr = [0i32; ARRAY_SIZE];
    println!("배열 크기: {} (상수 사용)", arr.len());

    println!("\n── 함수 실행 ──");
    println!("add(3, 4) = {}", add(3, 4));
    println!("factorial(10) = {}", factorial(10));
    println!("hot_path(5) = {}", hot_path(5));
    println!("cold_path(3) = {}", cold_path(3));

    println!("\n── 제로 코스트 추상화 ──");
    demonstrate_zero_cost();

    println!("\n── 컴파일 시간 정보 ──");
    // 컴파일 시각을 런타임에 삽입 (build.rs 없이 간단히)
    println!("이 파일: {}", file!());
    println!("이 줄:   {}", line!());
    println!("이 컬럼: {}", column!());
    println!("모듈:    {}", module_path!());

    println!("\n── 타입 크기와 레이아웃 ──");

    #[repr(C)]
    struct PointC {
        x: f32,
        y: f32,
        z: f32,
    }

    struct PointRust {
        x: f32,
        y: f32,
        z: f32,
    }

    #[repr(packed)]
    struct Packed {
        a: u8,
        b: u32,
        c: u8,
    }

    struct Normal {
        a: u8,
        b: u32,
        c: u8,
    }

    println!("PointC (repr C):    {} bytes", std::mem::size_of::<PointC>());
    println!("PointRust:          {} bytes", std::mem::size_of::<PointRust>());
    println!("Packed:             {} bytes (패딩 없음)", std::mem::size_of::<Packed>());
    println!("Normal:             {} bytes (패딩 포함)", std::mem::size_of::<Normal>());

    println!("\n── LLVM 최적화 레벨 ──");
    println!("Debug   (cargo build):          opt-level = 0, 디버그 심볼 있음");
    println!("Release (cargo build --release): opt-level = 3, 디버그 심볼 없음");
    println!("현재 빌드: {}", if cfg!(debug_assertions) { "Debug" } else { "Release" });

    println!("\n── 유용한 rustc 플래그 ──");
    let flags = [
        ("--emit mir",          "MIR 중간 표현 출력"),
        ("--emit llvm-ir",      "LLVM IR 출력"),
        ("--emit asm",          "어셈블리 출력"),
        ("-C opt-level=3",      "최고 최적화"),
        ("-C target-cpu=native","현재 CPU에 최적화"),
        ("--edition 2021",      "에디션 지정"),
        ("-Z time-passes",      "각 컴파일 단계 소요 시간 (nightly)"),
    ];

    for (flag, desc) in &flags {
        println!("  rustc {:25} # {}", flag, desc);
    }
}
