// 예제 01: 함수 기초 — 선언, 파라미터, 반환
// 실행: cargo run --example 01_functions_basics

// 기본 함수
fn greet(name: &str) {
    println!("안녕, {}!", name);
}

// 파라미터와 반환값
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 여러 반환값 (튜플)
fn divide(a: f64, b: f64) -> (f64, bool) {
    if b == 0.0 {
        (0.0, false)
    } else {
        (a / b, true)
    }
}

// Option으로 실패 가능한 연산
fn safe_sqrt(x: f64) -> Option<f64> {
    if x < 0.0 {
        None
    } else {
        Some(x.sqrt())
    }
}

// 슬라이스 파라미터 — 배열/벡터 모두 받을 수 있음
fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for &n in numbers {
        total += n;
    }
    total
}

fn max_value(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }
    let mut max = numbers[0];
    for &n in &numbers[1..] {
        if n > max {
            max = n;
        }
    }
    Some(max)
}

// 구조 분해 파라미터
fn print_point(&(x, y): &(i32, i32)) {
    println!("({}, {})", x, y);
}

// 기본값 패턴 (Rust에는 기본 파라미터 없음 — 오버로딩으로 모사)
fn connect(host: &str, port: u16) -> String {
    format!("{}:{}", host, port)
}

fn connect_default(host: &str) -> String {
    connect(host, 8080)
}

// 재귀 함수
fn power(base: i64, exp: u32) -> i64 {
    match exp {
        0 => 1,
        1 => base,
        n if n % 2 == 0 => {
            let half = power(base, n / 2);
            half * half
        }
        n => base * power(base, n - 1),
    }
}

// 함수를 값으로 (함수 포인터)
fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn double(x: i32) -> i32 { x * 2 }
fn square(x: i32) -> i32 { x * x }
fn negate(x: i32) -> i32 { -x }

// 중첩 함수 (함수 안의 함수)
fn outer(x: i32) -> i32 {
    fn inner(n: i32) -> i32 {
        n * n + 1
    }
    inner(x) + inner(x + 1)
}

// 다형성 — 제네릭 (Week 13에서 자세히)
fn swap<T: Copy>(a: T, b: T) -> (T, T) {
    (b, a)
}

// 가변 참조 파라미터
fn double_in_place(v: &mut Vec<i32>) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

fn main() {
    println!("=== 함수 기초 ===\n");

    // ── 기본 함수 호출 ─────────────────────────────────────────
    println!("── 1. 기본 함수 ──");
    greet("Rust");
    greet("세계");
    println!("add(3, 4) = {}", add(3, 4));
    println!("add(100, -50) = {}", add(100, -50));

    // ── 여러 반환값 ────────────────────────────────────────────
    println!("\n── 2. 여러 반환값 ──");
    let (result, ok) = divide(10.0, 3.0);
    if ok { println!("10 / 3 = {:.4}", result); }

    let (_, ok) = divide(5.0, 0.0);
    println!("5 / 0 성공 여부: {}", ok);

    // ── Option 반환 ────────────────────────────────────────────
    println!("\n── 3. Option 반환 ──");
    let roots = [4.0, 9.0, 16.0, -1.0, 0.0, 2.0];
    for x in roots {
        match safe_sqrt(x) {
            Some(r) => println!("sqrt({}) = {:.4}", x, r),
            None => println!("sqrt({}) = 정의되지 않음 (음수)", x),
        }
    }

    // ── 슬라이스 파라미터 ─────────────────────────────────────
    println!("\n── 4. 슬라이스 파라미터 ──");
    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let vec = vec![10, 20, 30, 40, 50];

    println!("sum(arr) = {}", sum(&arr));
    println!("sum(vec) = {}", sum(&vec));
    println!("sum([]) = {}", sum(&[]));

    println!("max(arr) = {:?}", max_value(&arr));
    println!("max([]) = {:?}", max_value(&[]));

    // ── 구조 분해 파라미터 ────────────────────────────────────
    println!("\n── 5. 구조 분해 파라미터 ──");
    let points = [(0, 0), (3, 4), (-1, 2)];
    for pt in &points {
        print_point(pt);
    }

    // ── 함수 포인터 ───────────────────────────────────────────
    println!("\n── 6. 함수 포인터 ──");
    let fns: Vec<fn(i32) -> i32> = vec![double, square, negate];
    let names = ["double", "square", "negate"];
    let x = 5;
    for (name, f) in names.iter().zip(fns.iter()) {
        println!("{}({}) = {}", name, x, apply(*f, x));
    }

    // 함수 포인터 배열 선택
    let ops: &[(&str, fn(i32, i32) -> i32)] = &[
        ("+", |a, b| a + b),
        ("-", |a, b| a - b),
        ("*", |a, b| a * b),
    ];
    for (sym, op) in ops {
        println!("3 {} 4 = {}", sym, op(3, 4));
    }

    // ── 재귀 함수 ─────────────────────────────────────────────
    println!("\n── 7. 재귀 함수 (거듭제곱) ──");
    for exp in 0..=10 {
        println!("2^{} = {}", exp, power(2, exp));
    }

    // ── 중첩 함수 ─────────────────────────────────────────────
    println!("\n── 8. 중첩 함수 ──");
    println!("outer(3) = {}", outer(3));  // inner(3) + inner(4) = 10 + 17 = 27
    println!("outer(5) = {}", outer(5));  // inner(5) + inner(6) = 26 + 37 = 63

    // ── 제네릭 ────────────────────────────────────────────────
    println!("\n── 9. 제네릭 함수 ──");
    let (a, b) = swap(1, 2);
    println!("swap(1, 2) = ({}, {})", a, b);
    let (s1, s2) = swap("hello", "world");
    println!("swap(\"hello\", \"world\") = (\"{}\", \"{}\")", s1, s2);

    // ── 가변 참조 ─────────────────────────────────────────────
    println!("\n── 10. 가변 참조 파라미터 ──");
    let mut nums = vec![1, 2, 3, 4, 5];
    println!("before: {:?}", nums);
    double_in_place(&mut nums);
    println!("after:  {:?}", nums);

    // ── 연결 ──────────────────────────────────────────────────
    println!("\n── 11. 연결 함수 ──");
    println!("{}", connect("localhost", 3000));
    println!("{}", connect_default("example.com"));
}
