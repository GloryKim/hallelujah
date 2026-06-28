// 예제 10: 타입 추론 심화
// 실행: cargo run --example 10_type_inference

fn main() {
    println!("=== Rust 타입 추론 심화 ===\n");

    // ── 기본 추론 ─────────────────────────────────────────────
    println!("── 1. 기본 타입 추론 ──");

    let a = 42;          // i32 (정수 기본)
    let b = 3.14;        // f64 (실수 기본)
    let c = true;        // bool
    let d = 'Z';         // char
    let e = "hello";     // &str

    // 타입 이름 출력 (제네릭 함수 트릭)
    fn type_name<T: ?Sized>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }
    println!("42        → {}", type_name(&a));
    println!("3.14      → {}", type_name(&b));
    println!("true      → {}", type_name(&c));
    println!("'Z'       → {}", type_name(&d));
    println!("\"hello\" → {}", type_name(&e));

    // ── 컨텍스트 기반 추론 ────────────────────────────────────
    println!("\n── 2. 컨텍스트 기반 추론 ──");

    // Vec에서 타입 어노테이션으로 추론
    let v: Vec<i64> = vec![1, 2, 3];   // 리터럴들이 i64로 추론됨
    println!("Vec<i64>: {:?}", v);

    // 사용처로부터 역방향 추론
    let mut sum = 0;        // i32로 추론 시작
    for &x in &v {
        sum += x as i32;    // sum이 i32로 확정됨
    }
    println!("합: {} (i32)", sum);

    // 클로저 타입 추론
    let double = |x| x * 2;        // 처음 호출 시 타입 확정
    let result = double(5i32);      // i32로 확정
    println!("double(5) = {}", result);
    // double(5.0);  // 오류! 이미 i32로 확정됨

    // ── 터보피쉬(::<>) ────────────────────────────────────────
    println!("\n── 3. 터보피쉬 문법 ──");

    // 어노테이션 없이 타입 지정
    let s1 = "42".parse::<i32>().unwrap();
    let s2 = "3.14".parse::<f64>().unwrap();
    let v1 = Vec::<String>::new();
    let v2 = Vec::<(i32, &str)>::new();

    println!("parse::<i32>: {}", s1);
    println!("parse::<f64>: {}", s2);
    println!("Vec::<String>: {:?}", v1);
    println!("Vec::<(i32, &str)>: {:?}", v2);

    // collect에서 필수
    let chars: Vec<char> = "hello".chars().collect();
    let words: std::collections::HashSet<&str> = "a b c a b".split_whitespace().collect();
    println!("chars: {:?}", chars);
    println!("words: {:?}", words);

    // ── 타입 어노테이션이 필요한 경우 ─────────────────────────
    println!("\n── 4. 명시적 어노테이션 필요 경우 ──");

    // 1) 범위 초과
    let big: i64 = 10_000_000_000;
    println!("i64 큰 수: {}", big);

    // 2) 파싱 — 반환 타입 지정
    let n: u32 = "255".parse().unwrap();
    println!("파싱: {}", n);

    // 3) collect — 목표 컬렉션 타입 지정
    let map: std::collections::HashMap<&str, i32> = vec![("a", 1), ("b", 2)]
        .into_iter()
        .collect();
    println!("HashMap: {:?}", map);

    // 4) 제네릭 함수 반환
    let zero = i32::default();         // 0
    let empty = String::default();     // ""
    println!("default i32: {}, String: {:?}", zero, empty);

    // ── 타입 별칭 ─────────────────────────────────────────────
    println!("\n── 5. 타입 별칭 ──");

    type Meters = f64;
    type Kilograms = f64;
    type Velocity = f64;  // m/s

    fn kinetic_energy(mass: Kilograms, velocity: Velocity) -> f64 {
        0.5 * mass * velocity * velocity
    }

    let mass: Kilograms = 70.0;
    let speed: Velocity = 5.0;
    let energy = kinetic_energy(mass, speed);
    println!("운동에너지({}kg, {}m/s) = {} J", mass, speed, energy);

    // 주의: 타입 별칭은 새 타입이 아님! 혼용 가능 (안전하지 않음)
    let dist: Meters = 100.0;
    let wt: Kilograms = 50.0;
    let _accidental_add = dist + wt;  // 경고 없음! Newtype 패턴(Week 30)으로 해결

    // ── 타입 변환 ─────────────────────────────────────────────
    println!("\n── 6. 타입 변환 ──");

    // as 캐스팅 (명시적, 잠재적 손실 가능)
    let large: i64 = 1000;
    let small = large as i8;   // 트런케이션 (오버플로!)
    println!("i64(1000) as i8 = {} (트런케이션)", small);

    let f = 3.99f64;
    let i = f as i32;  // 소수점 버림 (반올림 아님!)
    println!("f64(3.99) as i32 = {} (버림)", i);

    // From/Into (안전한 변환)
    let s: String = String::from("hello");
    let s2 = "world".to_string();
    let s3: String = "rust".into();
    println!("From: {}, to_string: {}, into: {}", s, s2, s3);

    let n: i64 = i32::MAX.into();  // i32 → i64 (손실 없음)
    println!("i32::MAX as i64: {}", n);

    // TryFrom/TryInto (실패 가능한 변환)
    let big: i64 = 1_000_000;
    match i32::try_from(big) {
        Ok(n) => println!("i64({}) → i32 성공: {}", big, n),
        Err(e) => println!("i64({}) → i32 실패: {}", big, e),
    }

    let too_big: i64 = i64::MAX;
    match i32::try_from(too_big) {
        Ok(n) => println!("성공: {}", n),
        Err(e) => println!("i64::MAX → i32 실패: {}", e),
    }

    // ── 타입 추론 한계 ─────────────────────────────────────────
    println!("\n── 7. 타입 추론이 안 되는 경우 ──");

    // 에러 예시 (주석 처리):
    // let v = Vec::new();   // 오류: 타입 모름
    // v.push(1);            // 이 줄이 있어야 타입 추론 가능

    let mut v = Vec::new();
    v.push(42i32);           // 이 줄로 타입 확정
    println!("타입 확정된 Vec: {:?} ({})", v, type_name(&v));

    // 또는 선언 시 명시:
    let v2: Vec<&str> = Vec::new();
    println!("명시적 타입: {:?} ({})", v2, type_name(&v2));
}
