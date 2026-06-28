// 예제 09: 팩토리얼 — 다양한 구현 방법 비교
// 실행: cargo run --example 09_factorial

// 1. 재귀 (단순)
fn factorial_recursive(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial_recursive(n - 1) }
}

// 2. 꼬리 재귀
fn factorial_tail(n: u64, acc: u64) -> u64 {
    if n == 0 { acc } else { factorial_tail(n - 1, n * acc) }
}

// 3. 반복문 (while)
fn factorial_while(n: u64) -> u64 {
    let mut result = 1u64;
    let mut i = 2u64;
    while i <= n {
        result *= i;
        i += 1;
    }
    result
}

// 4. 반복문 (for)
fn factorial_for(n: u64) -> u64 {
    let mut result = 1u64;
    for i in 2..=n {
        result *= i;
    }
    result
}

// 5. 이터레이터 (가장 관용적)
fn factorial_iter(n: u64) -> u64 {
    (1..=n).product()
}

// 6. fold
fn factorial_fold(n: u64) -> u64 {
    (1..=n).fold(1u64, |acc, x| acc * x)
}

// 7. 오버플로 안전 (checked)
fn factorial_checked(n: u64) -> Option<u64> {
    (1..=n).try_fold(1u64, |acc, x| acc.checked_mul(x))
}

// 8. 큰 수 (u128)
fn factorial_u128(n: u128) -> Option<u128> {
    (1..=n).try_fold(1u128, |acc, x| acc.checked_mul(x))
}

// 9. 이터레이터로 팩토리얼 수열 생성
struct FactorialIter {
    current_n: u64,
    current_val: u64,
}

impl FactorialIter {
    fn new() -> Self {
        FactorialIter { current_n: 0, current_val: 1 }
    }
}

impl Iterator for FactorialIter {
    type Item = (u64, u64);  // (n, n!)

    fn next(&mut self) -> Option<(u64, u64)> {
        let n = self.current_n;
        let val = self.current_val;
        self.current_n += 1;
        if n > 0 {
            self.current_val = self.current_val.checked_mul(n)?;
        }
        Some((n, val))
    }
}

// 10. 부동소수점 팩토리얼 (스털링 근사)
fn factorial_stirling(n: f64) -> f64 {
    // 스털링 근사: n! ≈ sqrt(2πn) * (n/e)^n
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    (2.0 * pi * n).sqrt() * (n / e).powf(n)
}

// 감마 함수 (실수 도메인 팩토리얼)
fn gamma_approx(n: f64) -> f64 {
    // Lanczos 근사 (간단 버전)
    if n < 0.5 {
        return std::f64::consts::PI / ((std::f64::consts::PI * n).sin() * gamma_approx(1.0 - n));
    }
    let n = n - 1.0;
    let coeffs = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];
    let mut x = coeffs[0];
    for (i, &c) in coeffs[1..].iter().enumerate() {
        x += c / (n + i as f64 + 1.0);
    }
    let t = n + 8.0 - 0.5;
    (2.0 * std::f64::consts::PI).sqrt() * t.powf(n + 0.5) * (-t).exp() * x
}

fn main() {
    println!("=== 팩토리얼 구현 비교 ===\n");

    // ── 1. 구현별 결과 비교 ───────────────────────────────────
    println!("── 1. 구현별 결과 비교 ──\n");

    println!("{:>4} {:>20} {:>20} {:>20}",
        "n", "재귀", "반복(for)", "이터레이터");
    println!("{}", "─".repeat(70));

    for n in 0u64..=12 {
        println!("{:>4} {:>20} {:>20} {:>20}",
            n,
            factorial_recursive(n),
            factorial_for(n),
            factorial_iter(n),
        );
    }

    // ── 2. u64 한계 ───────────────────────────────────────────
    println!("\n── 2. u64 오버플로 경계 ──");

    println!("20! = {}", factorial_checked(20).unwrap());
    println!("21! = {:?}", factorial_checked(21));  // None (u64 오버플로)

    // u128로 더 큰 수
    println!("\nu128 팩토리얼:");
    for n in [30u128, 34, 35] {
        match factorial_u128(n) {
            Some(v) => println!("{}! = {}", n, v),
            None => println!("{}! = 오버플로 (u128 범위 초과)", n),
        }
    }

    // ── 3. 팩토리얼 수열 이터레이터 ──────────────────────────
    println!("\n── 3. 팩토리얼 수열 ──");

    let seq: Vec<(u64, u64)> = FactorialIter::new().take(13).collect();
    for (n, val) in &seq {
        println!("  {}! = {}", n, val);
    }

    // 이터레이터 어댑터 활용
    let sum_of_factorials: u64 = FactorialIter::new()
        .take(8)
        .map(|(_, val)| val)
        .sum();
    println!("\n0! + 1! + ... + 7! = {}", sum_of_factorials);

    // ── 4. 근사 비교 ──────────────────────────────────────────
    println!("\n── 4. 스털링 근사 vs 정확값 ──");

    println!("{:>4} {:>25} {:>25} {:>10}", "n", "정확값", "스털링 근사", "오차%");
    println!("{}", "─".repeat(70));

    for n in [1u64, 5, 10, 15, 20] {
        let exact = factorial_iter(n) as f64;
        let approx = factorial_stirling(n as f64);
        let error = ((approx - exact) / exact * 100.0).abs();
        println!("{:>4} {:>25.0} {:>25.0} {:>9.4}%",
            n, exact, approx, error);
    }

    // ── 5. 이항 계수 (조합) ───────────────────────────────────
    println!("\n── 5. 이항 계수 C(n,k) = n! / (k! * (n-k)!) ──");

    fn choose(n: u64, k: u64) -> u64 {
        if k > n { return 0; }
        if k == 0 || k == n { return 1; }
        let k = k.min(n - k);  // 대칭성 활용
        (1..=k).fold(1u64, |acc, i| acc * (n - k + i) / i)
    }

    // 파스칼 삼각형
    println!("파스칼 삼각형 (처음 8행):");
    for n in 0..=7 {
        let row: Vec<u64> = (0..=n).map(|k| choose(n, k)).collect();
        let padding = " ".repeat((7 - n as usize) * 3);
        let values: Vec<String> = row.iter().map(|v| format!("{:>5}", v)).collect();
        println!("{}{}", padding, values.join(" "));
    }

    // ── 6. 성능 비교 ──────────────────────────────────────────
    println!("\n── 6. 성능 비교 (n=18) ──");

    let n = 18u64;
    let iterations = 1_000_000;

    let t0 = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = factorial_recursive(n);
    }
    println!("재귀:     {:?}", t0.elapsed());

    let t0 = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = factorial_while(n);
    }
    println!("while:    {:?}", t0.elapsed());

    let t0 = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = factorial_for(n);
    }
    println!("for:      {:?}", t0.elapsed());

    let t0 = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = factorial_iter(n);
    }
    println!("이터레이터: {:?}", t0.elapsed());

    let t0 = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = factorial_fold(n);
    }
    println!("fold:     {:?}", t0.elapsed());

    // ── 7. 검증 ───────────────────────────────────────────────
    println!("\n── 7. 모든 구현 일치 검증 ──");
    let mut all_match = true;
    for n in 0u64..=12 {
        let r = factorial_recursive(n);
        let results = [
            factorial_tail(n, 1),
            factorial_while(n),
            factorial_for(n),
            factorial_iter(n),
            factorial_fold(n),
            factorial_checked(n).unwrap(),
        ];
        if results.iter().any(|&x| x != r) {
            println!("{}! 불일치! {:?}", n, results);
            all_match = false;
        }
    }
    if all_match {
        println!("모든 구현이 n=0~12에서 일치합니다 ✓");
    }
}
