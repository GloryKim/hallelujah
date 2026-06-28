// 예제 08: 피보나치 수열 — 다양한 구현 방법
// 실행: cargo run --example 08_fibonacci

// 1. 재귀 (단순, 지수 시간 — 느림)
fn fib_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

// 2. 반복 (선형 시간 — 빠름)
fn fib_iterative(n: u64) -> u64 {
    if n == 0 { return 0; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

// 3. 꼬리 재귀 (Rust는 TCO 보장 안 하지만 명확한 표현)
fn fib_tail(n: u64, a: u64, b: u64) -> u64 {
    match n {
        0 => a,
        _ => fib_tail(n - 1, b, a + b),
    }
}

// 4. 메모이제이션 (HashMap 캐시)
fn fib_memo(n: u64) -> u64 {
    fn inner(n: u64, memo: &mut std::collections::HashMap<u64, u64>) -> u64 {
        if let Some(&v) = memo.get(&n) {
            return v;
        }
        let result = match n {
            0 => 0,
            1 => 1,
            _ => inner(n - 1, memo) + inner(n - 2, memo),
        };
        memo.insert(n, result);
        result
    }
    inner(n, &mut std::collections::HashMap::new())
}

// 5. 행렬 거듭제곱 (O(log n))
fn mat_mul(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
    [
        [
            a[0][0].wrapping_mul(b[0][0]).wrapping_add(a[0][1].wrapping_mul(b[1][0])),
            a[0][0].wrapping_mul(b[0][1]).wrapping_add(a[0][1].wrapping_mul(b[1][1])),
        ],
        [
            a[1][0].wrapping_mul(b[0][0]).wrapping_add(a[1][1].wrapping_mul(b[1][0])),
            a[1][0].wrapping_mul(b[0][1]).wrapping_add(a[1][1].wrapping_mul(b[1][1])),
        ],
    ]
}

fn mat_pow(mut m: [[u64; 2]; 2], mut n: u64) -> [[u64; 2]; 2] {
    let mut result = [[1, 0], [0, 1]];  // 단위 행렬
    while n > 0 {
        if n % 2 == 1 {
            result = mat_mul(result, m);
        }
        m = mat_mul(m, m);
        n /= 2;
    }
    result
}

fn fib_matrix(n: u64) -> u64 {
    if n == 0 { return 0; }
    let base = [[1u64, 1], [1, 0]];
    mat_pow(base, n)[0][1]
}

// 6. 이터레이터 스타일 — 수열 생성
struct FibIter {
    a: u64,
    b: u64,
}

impl FibIter {
    fn new() -> Self {
        FibIter { a: 0, b: 1 }
    }
}

impl Iterator for FibIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = self.a.checked_add(self.b)?;
        let current = self.a;
        self.a = self.b;
        self.b = next;
        Some(current)
    }
}

fn main() {
    println!("=== 피보나치 수열 구현 비교 ===\n");

    // ── 방법별 결과 비교 ──────────────────────────────────────
    println!("── 1. 구현별 결과 비교 (n=0~15) ──\n");
    println!("{:>4}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}",
        "n", "재귀", "반복", "꼬리재귀", "메모", "행렬");
    println!("{}", "─".repeat(60));

    for n in 0u64..=15 {
        println!("{:>4}  {:>10}  {:>10}  {:>10}  {:>10}  {:>10}",
            n,
            fib_recursive(n),
            fib_iterative(n),
            fib_tail(n, 0, 1),
            fib_memo(n),
            fib_matrix(n),
        );
    }

    // ── 큰 수 ────────────────────────────────────────────────
    println!("\n── 2. 큰 피보나치 수 (재귀는 제외) ──");
    for &n in &[30u64, 50, 70, 80, 93] {
        let v = fib_iterative(n);
        println!("F({:>3}) = {}", n, v);
    }
    println!("F(93) = {} (u64 최댓값 근접)", fib_iterative(93));
    println!("F(94) 는 u64 오버플로 (wrapping: {})", fib_matrix(94));

    // ── 이터레이터 활용 ───────────────────────────────────────
    println!("\n── 3. FibIter 이터레이터 활용 ──");

    // 처음 20개
    let first_20: Vec<u64> = FibIter::new().take(20).collect();
    println!("처음 20개:\n{:?}", first_20);

    // 짝수만 필터
    let even_fibs: Vec<u64> = FibIter::new()
        .take(20)
        .filter(|n| n % 2 == 0)
        .collect();
    println!("\n짝수 피보나치: {:?}", even_fibs);

    // 100 이하인 것들의 합
    let sum_below_100: u64 = FibIter::new()
        .take_while(|&n| n < 100)
        .sum();
    println!("100 미만 피보나치의 합: {}", sum_below_100);

    // 1000 미만 중 짝수의 합
    let even_sum: u64 = FibIter::new()
        .take_while(|&n| n < 1000)
        .filter(|n| n % 2 == 0)
        .sum();
    println!("1000 미만 짝수 피보나치의 합: {}", even_sum);

    // n번째 짝수 피보나치
    let nth_even = FibIter::new()
        .filter(|n| n % 2 == 0)
        .nth(5);
    println!("6번째 짝수 피보나치: {:?}", nth_even);

    // 백만 미만 최대 피보나치
    let max_under_million = FibIter::new()
        .take_while(|&n| n < 1_000_000)
        .last();
    println!("100만 미만 최대 피보나치: {:?}", max_under_million);

    // ── 수학적 성질 검증 ──────────────────────────────────────
    println!("\n── 4. 피보나치 수학적 성질 ──");

    // 황금 비율 수렴
    println!("황금비 수렴 (연속 두 항의 비율):");
    let fibs: Vec<u64> = FibIter::new().take(20).collect();
    for i in 5..15 {
        let ratio = fibs[i] as f64 / fibs[i - 1] as f64;
        println!("  F({}) / F({}) = {:.10}", i, i - 1, ratio);
    }
    println!("황금비 φ = {:.10}", (1.0 + 5.0f64.sqrt()) / 2.0);

    // 카시니 항등식: F(n-1) * F(n+1) - F(n)^2 = (-1)^n
    println!("\n카시니 항등식 검증:");
    for n in 1..=8usize {
        let f = |i: usize| fibs[i] as i64;
        let cassini = f(n - 1) * f(n + 1) - f(n) * f(n);
        let expected = if n % 2 == 0 { 1i64 } else { -1 };
        println!("  n={}: F({})*F({}) - F({})² = {} (예상: {})",
            n, n-1, n+1, n, cassini, expected);
    }

    // ── 성능 비교 ─────────────────────────────────────────────
    println!("\n── 5. 간단한 성능 측정 (F(35)) ──");

    let n = 35u64;

    let t0 = std::time::Instant::now();
    let r1 = fib_recursive(n);
    let t_recursive = t0.elapsed();

    let t0 = std::time::Instant::now();
    let r2 = fib_iterative(n);
    let t_iter = t0.elapsed();

    let t0 = std::time::Instant::now();
    let r3 = fib_matrix(n);
    let t_matrix = t0.elapsed();

    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
    println!("F({}) = {}", n, r1);
    println!("재귀:   {:?}", t_recursive);
    println!("반복:   {:?}", t_iter);
    println!("행렬:   {:?}", t_matrix);
}
