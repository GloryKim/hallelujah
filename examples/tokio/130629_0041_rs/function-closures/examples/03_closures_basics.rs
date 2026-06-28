// 예제 03: 클로저 기초 — 문법, 캡처 모드, 활용
// 실행: cargo run --example 03_closures_basics

fn main() {
    println!("=== 클로저 기초 ===\n");

    // ── 1. 클로저 문법 ────────────────────────────────────────
    println!("── 1. 클로저 문법 변형 ──");

    // 가장 간결한 형태
    let add = |a, b| a + b;

    // 타입 명시
    let add_typed = |a: i32, b: i32| a + b;

    // 블록 본문 (여러 줄)
    let add_verbose = |a: i32, b: i32| -> i32 {
        let result = a + b;
        result  // 세미콜론 없음 = 반환
    };

    // 일반 함수와 비교
    fn add_fn(a: i32, b: i32) -> i32 { a + b }

    println!("클로저 1: {}", add(3, 4));
    println!("클로저 2: {}", add_typed(3, 4));
    println!("클로저 3: {}", add_verbose(3, 4));
    println!("함수:     {}", add_fn(3, 4));

    // 파라미터 없는 클로저
    let hello = || println!("안녕!");
    hello();

    // 여러 줄 클로저
    let describe = |n: i32| {
        let sign = if n > 0 { "양수" } else if n < 0 { "음수" } else { "영" };
        let parity = if n % 2 == 0 { "짝수" } else { "홀수" };
        format!("{}: {} {}", n, sign, parity)
    };

    for n in [-3, 0, 4, 7] {
        println!("{}", describe(n));
    }

    // ── 2. 환경 캡처 ──────────────────────────────────────────
    println!("\n── 2. 환경 캡처 (공유 참조) ──");

    let threshold = 5;
    let message = String::from("큰 수");

    // 참조로 캡처 — 원본은 여전히 사용 가능
    let is_large = |x: i32| x > threshold;
    let describe_if_large = |x: i32| {
        if x > threshold {
            format!("{}: {}", x, message)
        } else {
            format!("{}: 작은 수", x)
        }
    };

    println!("is_large(3): {}", is_large(3));
    println!("is_large(8): {}", is_large(8));
    println!("{}", describe_if_large(3));
    println!("{}", describe_if_large(8));

    // 원본 여전히 유효
    println!("threshold: {}, message: {}", threshold, message);

    // ── 3. 가변 캡처 ──────────────────────────────────────────
    println!("\n── 3. 가변 캡처 ──");

    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };

    println!("1: {}", increment());
    println!("2: {}", increment());
    println!("3: {}", increment());

    // increment가 &mut count를 보유 중이므로
    // count를 직접 사용하려면 increment를 먼저 드롭해야 함
    drop(increment);  // 명시적으로 드롭
    println!("최종 count: {}", count);

    // 가변 클로저 패턴 — 상태 있는 클로저
    let mut accumulated = Vec::new();
    let mut push_if_odd = |x: i32| {
        if x % 2 != 0 {
            accumulated.push(x);
        }
    };

    for i in 1..=10 {
        push_if_odd(i);
    }
    drop(push_if_odd);
    println!("홀수만: {:?}", accumulated);

    // ── 4. 클로저 타입 ────────────────────────────────────────
    println!("\n── 4. 클로저 타입 (unique) ──");

    // 같은 코드를 쓰더라도 다른 클로저는 다른 타입
    let f = |x: i32| x + 1;
    let g = |x: i32| x + 1;  // f와 g는 다른 타입!

    // Vec에 같이 저장하려면 Box<dyn Fn> 필요
    let fns: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x - 3),
    ];

    let input = 10;
    for (i, func) in fns.iter().enumerate() {
        println!("fns[{}]({}) = {}", i, input, func(input));
    }

    // 타입 시스템으로 다른 함수 비교
    fn _takes_f(f: impl Fn(i32) -> i32) {}
    _takes_f(f);
    _takes_f(g);

    // ── 5. 클로저 파이프라인 ──────────────────────────────────
    println!("\n── 5. 클로저 파이프라인 ──");

    let operations: Vec<(&str, Box<dyn Fn(f64) -> f64>)> = vec![
        ("제곱", Box::new(|x| x * x)),
        ("루트", Box::new(|x| x.sqrt())),
        ("역수", Box::new(|x| 1.0 / x)),
        ("+10", Box::new(|x| x + 10.0)),
        ("*PI", Box::new(|x| x * std::f64::consts::PI)),
    ];

    let mut value = 4.0f64;
    println!("시작값: {}", value);
    for (name, op) in &operations {
        value = op(value);
        println!("  {} 후: {:.4}", name, value);
    }

    // ── 6. 클로저로 전략 패턴 ────────────────────────────────
    println!("\n── 6. 클로저로 전략 패턴 ──");

    fn sort_by_strategy<T: Clone>(mut data: Vec<T>,
        strategy: impl Fn(&T, &T) -> std::cmp::Ordering) -> Vec<T> {
        data.sort_by(strategy);
        data
    }

    let words = vec!["banana", "apple", "cherry", "date", "elderberry"];

    let by_length = sort_by_strategy(words.clone(), |a, b| a.len().cmp(&b.len()));
    println!("길이순: {:?}", by_length);

    let by_alpha = sort_by_strategy(words.clone(), |a, b| a.cmp(b));
    println!("알파벳순: {:?}", by_alpha);

    let by_rev = sort_by_strategy(words.clone(), |a, b| b.cmp(a));
    println!("역알파벳: {:?}", by_rev);

    // ── 7. 즉시 실행 클로저 ───────────────────────────────────
    println!("\n── 7. 즉시 실행 클로저 (IIFE) ──");

    let result = (|| {
        let x = 10;
        let y = 20;
        x + y
    })();
    println!("IIFE 결과: {}", result);

    // 복잡한 초기화에 유용
    let data = {
        let mut v = Vec::new();
        for i in 0..5 {
            v.push(i * i);
        }
        v
    };
    println!("초기화된 데이터: {:?}", data);

    // ── 8. 클로저 반환 ────────────────────────────────────────
    println!("\n── 8. 클로저 반환 (impl Fn) ──");

    fn make_greeter(greeting: &str) -> impl Fn(&str) -> String + '_ {
        move |name| format!("{}, {}!", greeting, name)
    }

    let hello = make_greeter("안녕하세요");
    let hi = make_greeter("Hi");
    let bye = make_greeter("잘 가요");

    println!("{}", hello("철수"));
    println!("{}", hi("Alice"));
    println!("{}", bye("영희"));

    fn make_range_checker(low: i32, high: i32) -> impl Fn(i32) -> bool {
        move |x| x >= low && x <= high
    }

    let is_teen = make_range_checker(13, 19);
    let is_adult = make_range_checker(18, 65);

    for age in [10, 15, 20, 65, 70] {
        println!("{}세: 청소년={}, 성인={}", age, is_teen(age), is_adult(age));
    }
}
