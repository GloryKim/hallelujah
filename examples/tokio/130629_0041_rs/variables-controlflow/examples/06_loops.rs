// 예제 06: 반복문 — loop, while, for 완전 가이드
// 실행: cargo run --example 06_loops

fn main() {
    println!("=== 반복문 완전 가이드 ===\n");

    // ════════════════════════════════════════════
    println!("── 1. loop — 명시적 break까지 반복 ──");
    // ════════════════════════════════════════════

    // 기본 loop
    let mut count = 0;
    loop {
        count += 1;
        if count >= 3 {
            println!("loop 종료, count = {}", count);
            break;
        }
    }

    // loop 표현식 — break에서 값 반환
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        if attempts * attempts >= 100 {
            break attempts;  // 값 반환
        }
    };
    println!("제곱이 100 이상인 첫 수: {} ({}²={})", result, result, result * result);

    // loop로 파싱 재시도 시뮬레이션
    let inputs = ["abc", "xyz", "42", "done"];
    let mut i = 0;
    let parsed = loop {
        if i >= inputs.len() {
            break None;
        }
        if let Ok(n) = inputs[i].parse::<i32>() {
            break Some(n);
        }
        i += 1;
    };
    println!("파싱 성공: {:?}", parsed);  // Some(42)

    // 중첩 loop와 레이블
    println!("\n중첩 loop 레이블:");
    let mut found = None;

    'search: for x in 1..=5 {
        for y in 1..=5 {
            if x * y == 12 {
                found = Some((x, y));
                break 'search;  // 바깥 루프까지 종료
            }
        }
    }
    println!("x*y=12 인 첫 쌍: {:?}", found);

    // continue 레이블
    'outer: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                continue 'outer;  // 바깥 루프의 다음 반복으로
            }
            print!("({},{}) ", i, j);
        }
    }
    println!("← j=1 건너뜀");

    // ════════════════════════════════════════════
    println!("\n── 2. while — 조건 반복 ──");
    // ════════════════════════════════════════════

    // 기본 while
    let mut n = 1;
    while n <= 10 {
        print!("{} ", n);
        n += 2;
    }
    println!("← 홀수");

    // 콜라츠 수열 (3n+1 문제)
    let mut num = 27u64;
    let mut steps = 0;
    print!("콜라츠(27): 27");
    while num != 1 {
        num = if num % 2 == 0 { num / 2 } else { 3 * num + 1 };
        steps += 1;
        if steps <= 5 || num <= 10 {
            print!(" → {}", num);
        } else if steps == 6 {
            print!(" → ...");
        }
    }
    println!(" ({}번 만에 1)", steps);

    // while let — 패턴이 맞는 동안 반복
    println!("\nwhile let:");
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("← 스택 비움");

    let mut optional = Some(0i32);
    while let Some(i) = optional {
        if i > 5 {
            println!("\n5 초과로 종료: {}", i);
            optional = None;
        } else {
            print!("{} ", i);
            optional = Some(i + 1);
        }
    }

    // ════════════════════════════════════════════
    println!("\n── 3. for — 반복자 기반 ──");
    // ════════════════════════════════════════════

    // 범위 반복
    println!("0..5: ");
    for i in 0..5 { print!("{} ", i); }
    println!();

    println!("0..=5: ");
    for i in 0..=5 { print!("{} ", i); }
    println!();

    println!("역순: ");
    for i in (0..5).rev() { print!("{} ", i); }
    println!();

    println!("짝수만: ");
    for i in (0..10).step_by(2) { print!("{} ", i); }
    println!();

    // 컬렉션 반복
    let fruits = ["사과", "바나나", "체리", "포도", "키위"];

    println!("\n참조로 반복 (소유권 유지):");
    for fruit in &fruits {
        print!("{} ", fruit);
    }
    println!();

    println!("인덱스 + 값:");
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  [{}] {}", i, fruit);
    }

    // 가변 참조로 반복
    let mut numbers = [10, 20, 30, 40, 50];
    for n in &mut numbers {
        *n += 5;  // 역참조로 수정
    }
    println!("5 더한 후: {:?}", numbers);

    // ── 반복자 어댑터와 for ───────────────────────────────────
    println!("\n── 4. 반복자 어댑터 (Week 3에서 자세히) ──");

    let data = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    // map
    let doubled: Vec<i32> = data.iter().map(|&x| x * 2).collect();
    println!("두 배: {:?}", doubled);

    // filter
    let evens: Vec<&i32> = data.iter().filter(|&&x| x % 2 == 0).collect();
    println!("짝수: {:?}", evens);

    // filter + map 연쇄
    let large_odd: Vec<i32> = data.iter()
        .filter(|&&x| x % 2 != 0)
        .filter(|&&x| x > 3)
        .map(|&x| x)
        .collect();
    println!("3 초과 홀수: {:?}", large_odd);

    // sum, product
    let sum: i32 = data.iter().sum();
    let product: i32 = [1, 2, 3, 4, 5].iter().product();
    println!("합계: {}, 곱: {}", sum, product);

    // ── 실용 패턴들 ───────────────────────────────────────────
    println!("\n── 5. 실용 패턴 ──");

    // 구구단
    println!("구구단 (5단):");
    for i in 1..=9 {
        print!("5 × {} = {:2}  ", i, 5 * i);
        if i % 3 == 0 { println!(); }
    }

    // 삼각형 출력
    println!("\n삼각형:");
    for i in 1..=5 {
        println!("{}", "*".repeat(i));
    }

    // 소수 판별 (에라토스테네스의 체 간단 버전)
    println!("\n50까지의 소수:");
    let mut primes = Vec::new();
    'outer: for n in 2..=50usize {
        for &p in &primes {
            if p * p > n { break; }
            if n % p == 0 { continue 'outer; }
        }
        primes.push(n);
    }
    println!("{:?}", primes);

    // 중첩 for — 행렬 출력
    println!("\n곱셈표 (1~5):");
    print!("    ");
    for j in 1..=5 { print!("{:4}", j); }
    println!();
    println!("    {}", "────".repeat(5));

    for i in 1..=5 {
        print!("{:2} |", i);
        for j in 1..=5 {
            print!("{:4}", i * j);
        }
        println!();
    }

    // enumerate + zip
    println!("\nzip 두 배열:");
    let names = ["Alice", "Bob", "Charlie"];
    let scores = [95, 87, 92];
    for (name, score) in names.iter().zip(scores.iter()) {
        println!("  {} → {}", name, score);
    }

    // flatten (중첩 반복자)
    let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("\nflatten: {:?}", flat);
}
