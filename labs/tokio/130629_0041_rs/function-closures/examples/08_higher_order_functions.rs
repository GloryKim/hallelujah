// 예제 08: 고차 함수 패턴 — 함수를 인자로/반환값으로
// 실행: cargo run --example 08_higher_order_functions

// ── 함수를 인자로 받는 함수들 ──────────────────────────────

fn apply<T, U, F: Fn(T) -> U>(f: F, x: T) -> U {
    f(x)
}

fn apply_twice<T: Copy, F: Fn(T) -> T>(f: F, x: T) -> T {
    f(f(x))
}

fn apply_n_times<T: Copy, F: Fn(T) -> T>(f: F, x: T, n: usize) -> T {
    let mut result = x;
    for _ in 0..n {
        result = f(result);
    }
    result
}

fn map_vec<T, U, F: Fn(T) -> U>(v: Vec<T>, f: F) -> Vec<U> {
    v.into_iter().map(f).collect()
}

fn filter_vec<T, F: Fn(&T) -> bool>(v: Vec<T>, pred: F) -> Vec<T> {
    v.into_iter().filter(|x| pred(x)).collect()
}

fn reduce<T: Copy, F: Fn(T, T) -> T>(v: &[T], f: F) -> Option<T> {
    let mut iter = v.iter();
    let first = iter.next().copied()?;
    Some(iter.fold(first, |acc, &x| f(acc, x)))
}

// ── 함수를 반환하는 함수들 ─────────────────────────────────

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

fn make_power(exp: u32) -> impl Fn(i32) -> i32 {
    move |x: i32| x.pow(exp)
}

fn make_in_range(lo: i32, hi: i32) -> impl Fn(i32) -> bool {
    move |x| x >= lo && x <= hi
}

// 함수 합성
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

// 여러 함수 합성 (벡터)
fn pipeline<T: Copy>(fns: &[Box<dyn Fn(T) -> T>], input: T) -> T {
    fns.iter().fold(input, |acc, f| f(acc))
}

// 함수를 저장하는 구조체
struct FunctionChain<T> {
    functions: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T: Clone + 'static> FunctionChain<T> {
    fn new() -> Self {
        FunctionChain { functions: Vec::new() }
    }

    fn then(mut self, f: impl Fn(T) -> T + 'static) -> Self {
        self.functions.push(Box::new(f));
        self
    }

    fn run(&self, input: T) -> T {
        self.functions.iter().fold(input, |acc, f| f(acc.clone()))
    }
}

// 커링 (currying)
fn curry_add(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

fn curry_mul(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a * b
}

// 메모이제이션
fn memoize<A: Clone + std::hash::Hash + Eq, B: Clone, F: Fn(A) -> B>(
    f: F,
) -> impl FnMut(A) -> B {
    let mut cache = std::collections::HashMap::new();
    move |x: A| {
        if let Some(result) = cache.get(&x) {
            result.clone()
        } else {
            let result = f(x.clone());
            cache.insert(x, result.clone());
            result
        }
    }
}

fn main() {
    println!("=== 고차 함수 패턴 ===\n");

    // ── 1. 기본 고차 함수 ────────────────────────────────────
    println!("── 1. 함수를 인자로 ──");

    println!("apply(double, 5) = {}", apply(|x: i32| x * 2, 5));
    println!("apply(square, 4) = {}", apply(|x: i32| x * x, 4));
    println!("apply_twice(+3, 7) = {}", apply_twice(|x| x + 3, 7));   // 13
    println!("apply_n_times(*2, 1, 5) = {}", apply_n_times(|x| x * 2, 1, 5));  // 32

    // ── 2. 함수형 컬렉션 처리 ────────────────────────────────
    println!("\n── 2. map/filter/reduce ──");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let doubled = map_vec(v.clone(), |x| x * 2);
    println!("두 배:    {:?}", doubled);

    let evens = filter_vec(v.clone(), |x| x % 2 == 0);
    println!("짝수:     {:?}", evens);

    println!("합:       {:?}", reduce(&v, |a, b| a + b));
    println!("최대:     {:?}", reduce(&v, |a, b| if a > b { a } else { b }));
    println!("최소:     {:?}", reduce(&v, |a, b| if a < b { a } else { b }));

    // ── 3. 클로저 팩토리 ──────────────────────────────────────
    println!("\n── 3. 클로저 팩토리 ──");

    let add5 = make_adder(5);
    let add10 = make_adder(10);
    let triple = make_multiplier(3);
    let cube = make_power(3);

    println!("add5(7) = {}", add5(7));
    println!("add10(7) = {}", add10(7));
    println!("triple(7) = {}", triple(7));
    println!("cube(4) = {}", cube(4));

    // 팩토리 함수로 여러 프레디케이트 생성
    let predicates: Vec<Box<dyn Fn(i32) -> bool>> = vec![
        Box::new(make_in_range(1, 10)),
        Box::new(make_in_range(5, 15)),
        Box::new(make_in_range(8, 20)),
    ];

    let test_values = [0, 5, 10, 15, 20];
    println!("\n범위 체크:");
    print!("{:>8}", "값");
    for (i, _) in predicates.iter().enumerate() {
        print!(" 범위{}", i + 1);
    }
    println!();
    for &v in &test_values {
        print!("{:>8}", v);
        for pred in &predicates {
            print!("  {:>4}", if pred(v) { "✓" } else { "✗" });
        }
        println!();
    }

    // ── 4. 함수 합성 ──────────────────────────────────────────
    println!("\n── 4. 함수 합성 (compose) ──");

    let add1_then_double = compose(|x: i32| x + 1, |x| x * 2);
    let double_then_add1 = compose(|x: i32| x * 2, |x| x + 1);

    for x in [0, 1, 5, 10] {
        println!("x={:2}: (+1 then *2)={:3}, (*2 then +1)={:3}",
            x, add1_then_double(x), double_then_add1(x));
    }

    // 여러 단계 합성
    let normalize = compose(
        compose(
            |s: String| s.trim().to_string(),
            |s: String| s.to_lowercase(),
        ),
        |s: String| s.replace(' ', "_"),
    );
    println!("\nnormalize(\"  Hello World  \") = {:?}",
        normalize("  Hello World  ".to_string()));

    // ── 5. 파이프라인 ─────────────────────────────────────────
    println!("\n── 5. 파이프라인 ──");

    let transforms: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 10),    // +10
        Box::new(|x| x * 2),     // *2
        Box::new(|x| x - 5),     // -5
        Box::new(|x| x / 3),     // /3
    ];

    for input in [0, 5, 10, 20] {
        let output = pipeline(&transforms, input);
        println!("{}→ +10→ *2→ -5→ /3 = {}", input, output);
    }

    // ── 6. FunctionChain 빌더 ────────────────────────────────
    println!("\n── 6. FunctionChain 빌더 ──");

    let chain = FunctionChain::new()
        .then(|x: i32| x + 5)
        .then(|x| x * 2)
        .then(|x| x.abs());

    println!("chain.run(3)  = {}", chain.run(3));   // (3+5)*2 = 16
    println!("chain.run(-8) = {}", chain.run(-8));  // (-8+5)*2 = |-6| = 6

    // ── 7. 커링 ───────────────────────────────────────────────
    println!("\n── 7. 커링 (Currying) ──");

    let add3 = curry_add(3);
    let add7 = curry_add(7);

    let nums = vec![1, 2, 3, 4, 5];
    let added3: Vec<i32> = nums.iter().map(|&x| add3(x)).collect();
    let added7: Vec<i32> = nums.iter().map(|&x| add7(x)).collect();

    println!("원본:    {:?}", nums);
    println!("+3:      {:?}", added3);
    println!("+7:      {:?}", added7);

    // ── 8. 메모이제이션 ───────────────────────────────────────
    println!("\n── 8. 메모이제이션 ──");

    let call_count = std::cell::Cell::new(0);
    let mut memoized_square = memoize(|x: i32| {
        call_count.set(call_count.get() + 1);
        x * x
    });

    let inputs = [5, 3, 5, 7, 3, 5, 7];
    print!("입력: {:?}\n결과: ", inputs);
    for &x in &inputs {
        print!("{} ", memoized_square(x));
    }
    println!("\n실제 계산 횟수: {} (중복 제거)", call_count.get());

    // ── 9. 실용 예제: 데이터 처리 파이프라인 ─────────────────
    println!("\n── 9. 실용 데이터 파이프라인 ──");

    #[derive(Debug, Clone)]
    struct Student {
        name: String,
        scores: Vec<i32>,
    }

    let students = vec![
        Student { name: "Alice".to_string(), scores: vec![85, 92, 78, 95] },
        Student { name: "Bob".to_string(), scores: vec![70, 65, 80, 75] },
        Student { name: "Charlie".to_string(), scores: vec![95, 98, 92, 99] },
        Student { name: "Dave".to_string(), scores: vec![55, 60, 58, 62] },
    ];

    let avg = |scores: &[i32]| -> f64 {
        scores.iter().sum::<i32>() as f64 / scores.len() as f64
    };

    let grade = |avg: f64| -> char {
        match avg as u32 {
            90..=100 => 'A',
            80..=89 => 'B',
            70..=79 => 'C',
            60..=69 => 'D',
            _ => 'F',
        }
    };

    let mut results: Vec<(String, f64, char)> = students.iter()
        .map(|s| {
            let a = avg(&s.scores);
            let g = grade(a);
            (s.name.clone(), a, g)
        })
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("{:<10} {:>7} {:>6}", "이름", "평균", "등급");
    println!("{}", "─".repeat(25));
    for (name, avg, grade) in &results {
        println!("{:<10} {:>7.1} {:>6}", name, avg, grade);
    }

    // 합격자 (평균 70 이상)
    let passed: Vec<_> = results.iter()
        .filter(|(_, avg, _)| *avg >= 70.0)
        .map(|(name, _, _)| name.as_str())
        .collect();
    println!("\n합격자: {:?}", passed);
}
