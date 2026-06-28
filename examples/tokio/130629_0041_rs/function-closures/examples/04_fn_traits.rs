// 예제 04: Fn / FnMut / FnOnce 트레이트 완전 가이드
// 실행: cargo run --example 04_fn_traits

// ─────────────────────────────────────────────
// FnOnce — 딱 한 번만 호출 가능 (값을 이동시킴)
// ─────────────────────────────────────────────
fn call_once<F: FnOnce() -> String>(f: F) -> String {
    f()
    // f()  // 두 번 호출하면 컴파일 오류!
}

// ─────────────────────────────────────────────
// FnMut — 여러 번 호출 가능, 환경 수정 가능
// ─────────────────────────────────────────────
fn call_n_times<F: FnMut() -> i32>(mut f: F, n: usize) -> Vec<i32> {
    (0..n).map(|_| f()).collect()
}

// ─────────────────────────────────────────────
// Fn — 여러 번 호출 가능, 환경 수정 불가 (순수)
// ─────────────────────────────────────────────
fn apply_to_all<F: Fn(i32) -> i32>(data: &[i32], f: F) -> Vec<i32> {
    data.iter().map(|&x| f(x)).collect()
}

fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

// ─────────────────────────────────────────────
// 트레이트 계층 이해
// FnOnce ⊃ FnMut ⊃ Fn
// ─────────────────────────────────────────────

// FnOnce를 받는 함수는 Fn/FnMut도 받을 수 있음
fn run_once<F: FnOnce() -> String>(f: F) {
    println!("결과: {}", f());
}

// Fn을 받는 함수에 FnMut/FnOnce를 전달하면 오류
fn needs_fn<F: Fn() -> i32>(f: F) -> i32 {
    f() + f()  // 두 번 호출 가능
}

// ─────────────────────────────────────────────
// Box<dyn Fn/FnMut/FnOnce> — 동적 디스패치
// ─────────────────────────────────────────────
type Callback = Box<dyn Fn(i32) -> i32>;

fn make_adder(n: i32) -> Callback {
    Box::new(move |x| x + n)
}

fn make_multiplier(n: i32) -> Callback {
    Box::new(move |x| x * n)
}

fn pipeline(callbacks: &[Callback], input: i32) -> i32 {
    callbacks.iter().fold(input, |acc, f| f(acc))
}

fn main() {
    println!("=== Fn / FnMut / FnOnce 트레이트 ===\n");

    // ── 1. FnOnce ─────────────────────────────────────────────
    println!("── 1. FnOnce (한 번만 호출) ──");

    let s = String::from("hello world");

    // String을 이동시키는 클로저 → FnOnce
    let consumer = move || {
        let words: Vec<&str> = s.split_whitespace().collect();
        format!("단어 수: {}", words.len())
    };

    println!("{}", call_once(consumer));
    // println!("{}", call_once(consumer));  // 오류! 이미 소비됨

    // run_once는 FnOnce를 받으므로 Fn도 통과
    run_once(|| String::from("Fn도 FnOnce로 전달 가능"));

    // ── 2. FnMut ──────────────────────────────────────────────
    println!("\n── 2. FnMut (여러 번 호출, 상태 변경) ──");

    // 카운터 클로저 (상태 있음)
    let mut counter = 0;
    let counter_fn = || {
        counter += 1;
        counter
    };

    let results = call_n_times(counter_fn, 5);
    println!("카운터 결과: {:?}", results);  // [1, 2, 3, 4, 5]

    // 누산기
    let mut sum = 0;
    let mut accumulate = |x: i32| {
        sum += x;
        sum
    };
    let running_sum: Vec<i32> = (1..=5).map(|x| accumulate(x)).collect();
    drop(accumulate);
    println!("누적 합: {:?}", running_sum);  // [1, 3, 6, 10, 15]
    println!("최종 합: {}", sum);

    // ID 생성기
    let mut id = 0u64;
    let mut next_id = || {
        id += 1;
        id
    };

    let ids: Vec<u64> = (0..5).map(|_| next_id()).collect();
    println!("생성된 ID: {:?}", ids);

    // ── 3. Fn ─────────────────────────────────────────────────
    println!("\n── 3. Fn (순수 함수, 여러 번 호출) ──");

    let multiplier = 3;
    let triple = |x: i32| x * multiplier;  // 참조로 캡처

    let data = vec![1, 2, 3, 4, 5];
    let tripled = apply_to_all(&data, triple);
    println!("3배: {:?}", tripled);

    // 여러 번 호출 가능
    println!("apply_twice(|x| x+3, 7) = {}", apply_twice(|x| x + 3, 7));  // 13
    println!("apply_twice(|x| x*2, 3) = {}", apply_twice(|x| x * 2, 3));  // 12

    // ── 4. 트레이트 계층 ──────────────────────────────────────
    println!("\n── 4. 트레이트 계층 (Fn ⊆ FnMut ⊆ FnOnce) ──");

    println!(
        r#"
  FnOnce: 환경 값을 이동할 수 있는 모든 클로저
    └─ FnMut: 환경을 수정하거나 읽는 클로저
          └─ Fn: 환경을 읽기만 하는 클로저

  Fn은 FnMut의 부분집합 (Fn이면 자동으로 FnMut도 됨)
  FnMut은 FnOnce의 부분집합 (FnMut이면 자동으로 FnOnce도 됨)
"#
    );

    // Fn 클로저를 FnMut 파라미터에 전달 — OK
    let pure_fn = |x: i32| x + 1;
    let results = call_n_times(pure_fn, 3);
    println!("Fn → FnMut에 전달: {:?}", results);

    // Fn 클로저를 FnOnce 파라미터에 전달 — OK
    run_once(|| String::from("Fn → FnOnce에 전달"));

    // needs_fn은 두 번 호출하므로 Fn 필요
    let result = needs_fn(|| 21);
    println!("needs_fn(|| 21) = {}", result);  // 42

    // ── 5. 실용 예제: 이벤트 핸들러 ─────────────────────────
    println!("\n── 5. 이벤트 핸들러 시스템 ──");

    struct EventEmitter {
        handlers: Vec<Box<dyn Fn(&str)>>,
    }

    impl EventEmitter {
        fn new() -> Self {
            EventEmitter { handlers: Vec::new() }
        }

        fn on(&mut self, handler: impl Fn(&str) + 'static) {
            self.handlers.push(Box::new(handler));
        }

        fn emit(&self, event: &str) {
            for handler in &self.handlers {
                handler(event);
            }
        }
    }

    let mut emitter = EventEmitter::new();

    emitter.on(|e| println!("  핸들러 1: {}", e));
    emitter.on(|e| println!("  핸들러 2: {} (처리됨)", e.to_uppercase()));

    let prefix = "LOG";
    emitter.on(move |e| println!("  [{}] {}", prefix, e));

    emitter.emit("클릭");
    emitter.emit("데이터 로드");

    // ── 6. Box<dyn Fn> 파이프라인 ────────────────────────────
    println!("\n── 6. 동적 디스패치 파이프라인 ──");

    let transforms: Vec<Callback> = vec![
        make_adder(5),
        make_multiplier(2),
        make_adder(-3),
        make_multiplier(10),
    ];

    let input = 7;
    let output = pipeline(&transforms, input);
    // (7 + 5) * 2 - 3 = 21, 21 * 10 = 210
    println!("입력: {}, 출력: {}", input, output);
    println!("단계: {} → +5={} → *2={} → -3={} → *10={}",
        input, input+5, (input+5)*2, (input+5)*2-3, ((input+5)*2-3)*10);

    // ── 7. 클로저 저장 ────────────────────────────────────────
    println!("\n── 7. 구조체에 클로저 저장 ──");

    struct Transformer<F: Fn(i32) -> i32> {
        func: F,
        name: String,
    }

    impl<F: Fn(i32) -> i32> Transformer<F> {
        fn new(name: &str, func: F) -> Self {
            Transformer { func, name: name.to_string() }
        }

        fn apply(&self, x: i32) -> i32 {
            (self.func)(x)
        }
    }

    let doubler = Transformer::new("두 배", |x| x * 2);
    let squarer = Transformer::new("제곱", |x| x * x);

    for x in [1, 2, 5, 10] {
        println!("{}({}) = {}", doubler.name, x, doubler.apply(x));
        println!("{}({}) = {}", squarer.name, x, squarer.apply(x));
    }
}
