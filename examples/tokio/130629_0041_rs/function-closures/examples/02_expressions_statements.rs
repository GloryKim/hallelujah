// 예제 02: 표현식 vs 문장 — Rust의 핵심 개념
// 실행: cargo run --example 02_expressions_statements

fn five() -> i32 {
    5       // 세미콜론 없음 = 표현식 = 반환값
}

fn six() -> i32 {
    let x = 5;
    x + 1   // 마지막 표현식이 반환값
}

fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= c {
        b
    } else {
        c
    }
}

fn classify(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "음수",
        0 => "영",
        1..=9 => "한 자리",
        10..=99 => "두 자리",
        _ => "세 자리 이상",
    }
}

fn main() {
    println!("=== 표현식 vs 문장 ===\n");

    // ── 문장 (Statement) ──────────────────────────────────────
    println!("── 1. 문장 ──");
    // 문장은 값을 반환하지 않음
    let x = 5;              // 선언문 (;로 끝남)
    println!("{}", x);      // 매크로 문 (;로 끝남)
    let _y = {              // 블록도 문장이 될 수 있음
        let z = 10;
        z + 5;              // ; 가 있으면 () 반환
    };
    // _y == ()

    // ── 표현식 (Expression) ───────────────────────────────────
    println!("\n── 2. 표현식 ──");

    // 리터럴 — 표현식
    let _ = 42;
    let _ = "hello";
    let _ = true;

    // 연산자 — 표현식
    let sum = 3 + 4 * 2;    // 11
    println!("3 + 4 * 2 = {}", sum);

    // 블록 표현식 ({}에서 마지막 표현식이 반환값)
    let result = {
        let base = 10;
        let height = 5;
        base * height / 2  // 세미콜론 없음 → 25 반환
    };
    println!("블록 표현식: {}", result);

    // if 표현식
    let abs_val = if sum > 0 { sum } else { -sum };
    println!("절댓값: {}", abs_val);

    // match 표현식
    let description = match sum {
        0 => "영",
        1..=9 => "한 자리",
        10..=99 => "두 자리",
        _ => "큰 수",
    };
    println!("match 결과: {}", description);

    // loop 표현식
    let mut count = 0;
    let loop_result = loop {
        count += 1;
        if count == 10 {
            break count * count;  // 100 반환
        }
    };
    println!("loop 결과: {}", loop_result);

    // ── 함수 반환값 ───────────────────────────────────────────
    println!("\n── 3. 함수 반환값 ──");
    println!("five() = {}", five());
    println!("six() = {}", six());
    println!("max_of_three(3, 7, 5) = {}", max_of_three(3, 7, 5));

    // ── 표현식의 중첩 ─────────────────────────────────────────
    println!("\n── 4. 표현식 중첩 ──");

    // 모든 것이 표현식이므로 연쇄 가능
    let complex = {
        let base = if true { 10 } else { 20 };
        let multiplier = match base {
            10 => 3,
            _ => 1,
        };
        let result = loop {
            break base * multiplier;
        };
        result + 5  // 35
    };
    println!("복잡한 중첩 표현식: {}", complex);

    // ── 실용 패턴 ─────────────────────────────────────────────
    println!("\n── 5. 실용 패턴 ──");

    // 1) 초기화 표현식
    let message = format!("결과: {}", 42 * 2);
    println!("{}", message);

    // 2) 조기 반환을 if로 대체
    fn parse_age(s: &str) -> Option<u32> {
        let n: u32 = s.trim().parse().ok()?;
        if n > 150 { None } else { Some(n) }
    }

    for s in ["25", "200", "abc", " 18 "] {
        println!("parse_age({:?}) = {:?}", s, parse_age(s));
    }

    // 3) 블록으로 스코프 제어
    let config = {
        let mut map = std::collections::HashMap::new();
        map.insert("host", "localhost");
        map.insert("port", "8080");
        map.insert("debug", "true");
        map  // 불변으로 반환
    };
    println!("설정: {:?}", config);

    // 4) 각 숫자 분류 (match 표현식)
    println!("\n숫자 분류:");
    for n in [-100, -1, 0, 1, 9, 10, 99, 100, 1000] {
        println!("  {} → {}", n, classify(n));
    }

    // ── 세미콜론의 의미 ───────────────────────────────────────
    println!("\n── 6. 세미콜론의 의미 ──");

    // ; 있음 → 표현식을 문장으로 변환 → () 반환
    let a: () = { 5; };      // {} 안에 5; 가 있으면 () 반환
    let _b: i32 = { 5 };     // {} 안에 5 (세미콜론 없음) 이면 5 반환

    println!("세미콜론 있는 블록 타입: {:?}", a);
    println!("세미콜론 없는 블록 타입: {}", _b);

    // 함수에서의 차이
    fn with_semicolon() {  // 반환 타입 없음 = ()
        let _ = 5;
        42;  // 이 값은 버려짐
    }
    fn without_semicolon() -> i32 {
        42   // 이 값이 반환됨
    }
    with_semicolon();
    println!("without_semicolon() = {}", without_semicolon());

    // ── 단락 연산자 ───────────────────────────────────────────
    println!("\n── 7. 단락 연산자 (? 연산자) ──");

    fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n: i32 = s.trim().parse()?;  // 실패 시 즉시 Err 반환
        Ok(n * 2)  // Ok로 감싸서 반환
    }

    for s in ["21", "  7  ", "abc", "999"] {
        match parse_and_double(s) {
            Ok(n) => println!("  {:?} → {}", s, n),
            Err(e) => println!("  {:?} → 오류: {}", s, e),
        }
    }
}
