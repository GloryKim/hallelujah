// 예제 05: move 클로저 — 소유권 이전과 수명
// 실행: cargo run --example 05_move_closures

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    // x는 스택에 있음. move 없으면 x의 참조가 dangling
    move |y| x + y  // x를 클로저로 이동
}

fn make_greeting(prefix: String) -> impl Fn(&str) -> String {
    move |name| format!("{}, {}!", prefix, name)
}

fn make_predicate(threshold: i32) -> impl Fn(i32) -> bool {
    move |x| x > threshold
}

fn apply_later<F: Fn() -> String + Send + 'static>(f: F) -> String {
    // 'static 요구: f는 어떤 참조도 캡처하면 안 됨
    // (또는 'static 수명의 참조만 OK)
    f()
}

fn main() {
    println!("=== move 클로저 ===\n");

    // ── 1. 왜 move가 필요한가? ────────────────────────────────
    println!("── 1. move가 필요한 이유 ──");

    let x = 10;
    // move 없는 클로저 — 참조로 캡처
    let borrow_fn = || x + 1;
    println!("borrow_fn(): {}", borrow_fn());
    println!("x 여전히 사용 가능: {}", x);  // OK

    // move 있는 클로저 — 값을 이동
    let y = 20;
    let owned_fn = move || y + 1;
    println!("owned_fn(): {}", owned_fn());
    println!("y 여전히 사용 가능: {}", y);  // Copy 타입이라 OK!
    // (i32는 Copy이므로 move해도 y는 계속 사용 가능)

    // String은 Copy가 아님 — move 후 사용 불가
    let s = String::from("hello");
    let consume = move || format!("{}!", s);
    println!("consume(): {}", consume());
    // println!("{}", s);  // 오류! s가 이동됨

    // ── 2. 클로저를 반환할 때 move 필수 ─────────────────────
    println!("\n── 2. 클로저 반환 ──");

    let add5 = make_adder(5);
    let add10 = make_adder(10);

    for x in [1, 2, 3, 4, 5] {
        print!("add5({})={}, add10({})={}  ", x, add5(x), x, add10(x));
    }
    println!();

    let greet = make_greeting(String::from("안녕하세요"));
    let hi = make_greeting(String::from("Hi"));

    println!("{}", greet("철수"));
    println!("{}", hi("Alice"));

    let is_positive = make_predicate(0);
    let is_teen = make_predicate(12);

    let ages = [8, 13, 25, 15, 5];
    for &age in &ages {
        println!("{}세: 양수={}, 청소년이상={}", age, is_positive(age), is_teen(age));
    }

    // ── 3. 스레드에서 move 필수 ───────────────────────────────
    println!("\n── 3. 스레드에서 move ──");

    let data = vec![1, 2, 3, 4, 5];
    let message = String::from("작업 완료");

    // move 없으면 컴파일 오류:
    // 스레드는 data보다 오래 살 수 있음 → 참조 dangling 위험
    let handle = std::thread::spawn(move || {
        // data와 message가 이 스레드로 이동됨
        let sum: i32 = data.iter().sum();
        println!("스레드 내부: 합={}, {}", sum, message);
        sum
    });

    let result = handle.join().unwrap();
    println!("스레드 결과: {}", result);

    // ── 4. move + 참조의 조합 ─────────────────────────────────
    println!("\n── 4. move + 참조 조합 ──");

    let numbers = vec![10, 20, 30, 40, 50];

    // 클로저가 Vec의 참조를 캡처하면 Vec보다 오래 살 수 없음
    let sum_fn = || numbers.iter().sum::<i32>();
    println!("합: {}", sum_fn());
    println!("원본: {:?}", numbers);  // OK — 참조만 캡처

    // move로 Vec 자체를 이동
    let sum_owned = move || numbers.iter().sum::<i32>();
    println!("move 합: {}", sum_owned());
    // println!("{:?}", numbers);  // 오류! numbers가 이동됨

    // ── 5. 'static 수명 요구 ─────────────────────────────────
    println!("\n── 5. 'static 수명 ──");

    // 정적 문자열은 'static
    let static_result = apply_later(|| String::from("정적 데이터"));
    println!("{}", static_result);

    // 힙 데이터를 move한 클로저는 'static
    let owned_data = String::from("힙 데이터");
    let result = apply_later(move || format!("처리: {}", owned_data));
    println!("{}", result);

    // ── 6. move 클로저 캡처 확인 ─────────────────────────────
    println!("\n── 6. 캡처 모드 비교 ──");

    // 참조 캡처 — 공유 가능
    let nums = vec![1, 2, 3];
    let borrow1 = || nums.iter().sum::<i32>();
    let borrow2 = || nums.iter().count();  // 둘 다 &nums 보유 가능
    println!("합: {}, 길이: {}", borrow1(), borrow2());
    println!("원본: {:?}", nums);

    // move 캡처 — 소유권 이동
    let owned = vec![1, 2, 3];
    let _moved = move || owned.len();
    // 아래 줄은 오류:
    // println!("{:?}", owned);  // owned가 _moved로 이동됨

    // ── 7. move 클로저 저장 ───────────────────────────────────
    println!("\n── 7. move 클로저 컬렉션 저장 ──");

    let mut closures: Vec<Box<dyn Fn() -> i32>> = Vec::new();

    for i in 0..5 {
        // i를 move로 캡처하지 않으면 루프 후 dangling!
        closures.push(Box::new(move || i * i));
    }

    for (i, f) in closures.iter().enumerate() {
        println!("closures[{}]() = {}", i, f());
    }

    // ── 8. 실용 예제: 지연 계산 ──────────────────────────────
    println!("\n── 8. 지연 계산 (Thunk) ──");

    struct Lazy<T, F: FnOnce() -> T> {
        value: Option<T>,
        factory: Option<F>,
    }

    impl<T, F: FnOnce() -> T> Lazy<T, F> {
        fn new(f: F) -> Self {
            Lazy { value: None, factory: Some(f) }
        }

        fn get(&mut self) -> &T {
            if self.value.is_none() {
                println!("  (처음 접근 — 지금 계산)");
                self.value = Some(self.factory.take().unwrap()());
            }
            self.value.as_ref().unwrap()
        }
    }

    let mut lazy_data = Lazy::new(move || {
        // 비싼 계산 시뮬레이션
        let v: Vec<i32> = (1..=100).map(|x| x * x).collect();
        v.iter().sum::<i32>()
    });

    println!("지연 값 첫 접근:");
    println!("  결과 = {}", lazy_data.get());
    println!("지연 값 두 번째 접근 (캐시됨):");
    println!("  결과 = {}", lazy_data.get());
}
