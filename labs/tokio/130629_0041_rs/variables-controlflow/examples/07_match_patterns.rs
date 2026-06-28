// 예제 07: match — 패턴 매칭 완전 가이드
// 실행: cargo run --example 07_match_patterns

#[derive(Debug)]
#[allow(dead_code)]  // 예제용 — 모든 variant가 match에서 사용됨을 보여주기 위해 정의
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "원",
            Shape::Rectangle(_, _) => "직사각형",
            Shape::Triangle(_, _, _) => "삼각형",
        }
    }
}

fn main() {
    println!("=== match 패턴 매칭 완전 가이드 ===\n");

    // ── 1. 기본 match ──────────────────────────────────────────
    println!("── 1. 기본 match ──");

    let n = 13;
    match n {
        1 => println!("하나"),
        2 => println!("둘"),
        3 | 4 | 5 => println!("3, 4, 또는 5"),   // OR 패턴
        6..=10 => println!("6에서 10"),            // 범위 패턴
        11..=20 => println!("11에서 20"),
        _ => println!("그 외"),                    // 와일드카드 (필수!)
    }

    // match는 표현식
    let description = match n {
        1 => "one",
        2 => "two",
        3..=9 => "single digit (3+)",
        _ => "large",
    };
    println!("{} = {}", n, description);

    // ── 2. 튜플 match ─────────────────────────────────────────
    println!("\n── 2. 튜플 패턴 ──");

    let point = (1, -1);
    match point {
        (0, 0) => println!("원점"),
        (x, 0) => println!("x축: x={}", x),
        (0, y) => println!("y축: y={}", y),
        (x, y) if x == y => println!("y=x 선 위: ({},{})", x, y),
        (x, y) if x == -y => println!("y=-x 선 위: ({},{})", x, y),
        (x, y) if x > 0 && y > 0 => println!("1사분면: ({},{})", x, y),
        (x, y) if x < 0 && y > 0 => println!("2사분면: ({},{})", x, y),
        (x, y) if x < 0 && y < 0 => println!("3사분면: ({},{})", x, y),
        (x, y) => println!("4사분면: ({},{})", x, y),
    }

    // ── 3. 가드 (Guard) ───────────────────────────────────────
    println!("\n── 3. 매치 가드 (if) ──");

    let numbers = [1, 2, 3, 15, 20, 100, -5];
    for &num in &numbers {
        let label = match num {
            n if n < 0 => "음수",
            0 => "영",
            n if n % 15 == 0 => "15의 배수",
            n if n % 3 == 0 => "3의 배수",
            n if n % 5 == 0 => "5의 배수",
            _ => "기타",
        };
        println!("  {} → {}", num, label);
    }

    // ── 4. 구조체 구조 분해 ───────────────────────────────────
    println!("\n── 4. 구조체 구조 분해 ──");

    struct Point { x: i32, y: i32 }
    struct Color { r: u8, g: u8, b: u8 }

    let p = Point { x: 3, y: -7 };
    match p {
        Point { x: 0, y: 0 } => println!("원점"),
        Point { x, y: 0 } => println!("x축, x={}", x),
        Point { x: 0, y } => println!("y축, y={}", y),
        Point { x, y } => println!("({}, {})", x, y),
    }

    let c = Color { r: 0, g: 128, b: 255 };
    match c {
        Color { r: 0, g: 0, b: 0 } => println!("검정"),
        Color { r: 255, g: 255, b: 255 } => println!("흰색"),
        Color { r: 255, g: 0, b: 0 } => println!("빨강"),
        Color { r, g, b } => println!("RGB({}, {}, {})", r, g, b),
    }

    // ── 5. 열거형 match ───────────────────────────────────────
    println!("\n── 5. 열거형 패턴 ──");

    let dir = Direction::North;
    let move_vector = match dir {
        Direction::North => (0, 1),
        Direction::South => (0, -1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
    };
    println!("North → 벡터: {:?}", move_vector);

    // Shape enum
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in &shapes {
        println!("{}: 넓이 = {:.2}", shape.name(), shape.area());
    }

    // ── 6. Option / Result match ──────────────────────────────
    println!("\n── 6. Option과 Result ──");

    let numbers: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    for (i, opt) in numbers.iter().enumerate() {
        match opt {
            Some(n) => println!("  [{}] = {}", i, n),
            None => println!("  [{}] = (없음)", i),
        }
    }

    let results: Vec<Result<i32, &str>> = vec![
        Ok(10),
        Err("실패"),
        Ok(30),
    ];
    let sum: i32 = results.iter()
        .filter_map(|r| r.as_ref().ok().copied())
        .sum();
    println!("Ok 값들의 합: {}", sum);

    // ── 7. @ 바인딩 ───────────────────────────────────────────
    println!("\n── 7. @ 바인딩 ──");

    let age = 15u32;
    match age {
        n @ 0..=12 => println!("어린이 ({}세)", n),
        n @ 13..=17 => println!("청소년 ({}세)", n),
        n @ 18..=64 => println!("성인 ({}세)", n),
        n => println!("노인 ({}세)", n),
    }

    // ── 8. 중첩 패턴 ──────────────────────────────────────────
    println!("\n── 8. 중첩 패턴 ──");

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        Color(u8, u8, u8),
    }

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("안녕")),
        Message::Color(255, 128, 0),
    ];

    for msg in &messages {
        match msg {
            Message::Quit => println!("종료"),
            Message::Move { x, y } => println!("이동: ({}, {})", x, y),
            Message::Write(text) => println!("쓰기: {}", text),
            Message::Color(r, g, b) => println!("색상: rgb({},{},{})", r, g, b),
        }
    }

    // ── 9. 슬라이스 패턴 ──────────────────────────────────────
    println!("\n── 9. 슬라이스 패턴 ──");

    let arrays: Vec<Vec<i32>> = vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 2, 3, 4, 5],
    ];

    for arr in &arrays {
        match arr.as_slice() {
            [] => println!("빈 배열"),
            [x] => println!("단일 요소: {}", x),
            [x, y] => println!("두 요소: {}, {}", x, y),
            [first, .., last] => println!("첫={}, 마지막={}, 총{}개", first, last, arr.len()),
        }
    }

    // ── 10. if let / while let ────────────────────────────────
    println!("\n── 10. if let / while let ──");

    // if let — 한 패턴만 처리할 때
    let val: Option<i32> = Some(42);
    if let Some(n) = val {
        println!("if let Some: {}", n);
    }

    // if let ... else
    let result: Result<i32, &str> = Err("오류");
    if let Ok(n) = result {
        println!("성공: {}", n);
    } else {
        println!("실패 처리");
    }

    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("← while let pop");
}
