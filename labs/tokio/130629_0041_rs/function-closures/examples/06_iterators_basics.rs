// 예제 06: 이터레이터 기초 — iter, iter_mut, into_iter
// 실행: cargo run --example 06_iterators_basics

// 커스텀 Iterator 구현
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 피보나치 이터레이터
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let current = self.a;
        let next = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = next;
        Some(current)
    }
}

// 무한 범위 이터레이터
struct Range {
    current: i32,
    step: i32,
}

impl Range {
    fn from(start: i32, step: i32) -> Self {
        Range { current: start, step }
    }
}

impl Iterator for Range {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let val = self.current;
        self.current += self.step;
        Some(val)  // 무한 — None 반환 안 함
    }
}

fn main() {
    println!("=== 이터레이터 기초 ===\n");

    // ── 1. 세 가지 반복 메서드 비교 ──────────────────────────
    println!("── 1. iter() / iter_mut() / into_iter() ──\n");

    let v = vec![1, 2, 3, 4, 5];

    // iter() — 불변 참조 (&T) 반환, 원본 유지
    print!("iter():     ");
    for x in v.iter() {
        print!("{:?} ", x);  // x: &i32
    }
    println!("← &i32");
    println!("원본 유지: {:?}", v);

    // iter_mut() — 가변 참조 (&mut T) 반환
    let mut v2 = vec![1, 2, 3, 4, 5];
    for x in v2.iter_mut() {
        *x *= 10;  // x: &mut i32
    }
    print!("iter_mut(): ");
    for x in &v2 { print!("{} ", x); }
    println!("← 수정됨");

    // into_iter() — 소유권 이동 (T) 반환
    let v3 = vec!["a", "b", "c"];
    print!("into_iter():");
    for s in v3.into_iter() {
        print!(" {}", s);  // s: &str
    }
    println!("← 원본 이동됨");
    // println!("{:?}", v3);  // 오류! v3 이동됨

    // for 루프에서의 암시적 into_iter
    println!("\n── for 루프의 암시적 변환 ──");
    let data = vec![10, 20, 30];

    // &data → data.iter() 와 동일
    for x in &data {
        print!("{} ", x);
    }
    println!("← &data → iter()");

    // &mut data → data.iter_mut() 와 동일
    let mut data2 = vec![10, 20, 30];
    for x in &mut data2 {
        *x += 1;
    }
    println!("{:?} ← &mut data → iter_mut()", data2);

    // data → data.into_iter() 와 동일
    let data3 = vec![10, 20, 30];
    for x in data3 {  // data3 이동
        print!("{} ", x);
    }
    println!("← data → into_iter()");

    // ── 2. Iterator 트레이트 ──────────────────────────────────
    println!("\n── 2. Iterator 트레이트 — next() 직접 호출 ──");

    let mut iter = vec![10, 20, 30].into_iter();
    println!("next(): {:?}", iter.next());  // Some(10)
    println!("next(): {:?}", iter.next());  // Some(20)
    println!("next(): {:?}", iter.next());  // Some(30)
    println!("next(): {:?}", iter.next());  // None
    println!("next(): {:?}", iter.next());  // None (이후에도 계속 None)

    // ── 3. 커스텀 이터레이터 ──────────────────────────────────
    println!("\n── 3. 커스텀 이터레이터 ──");

    // Counter
    let counter = Counter::new(5);
    let collected: Vec<u32> = counter.collect();
    println!("Counter(5): {:?}", collected);

    // Counter로 어댑터 사용 (Iterator 트레이트 자동 제공)
    let sum: u32 = Counter::new(5).sum();
    let doubled: Vec<u32> = Counter::new(5).map(|x| x * 2).collect();
    println!("합: {}, 두 배: {:?}", sum, doubled);

    // Fibonacci
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("피보나치 10개: {:?}", fibs);

    let fib_sum: u64 = Fibonacci::new()
        .take_while(|&n| n < 100)
        .sum();
    println!("100 미만 피보나치 합: {}", fib_sum);

    // 무한 이터레이터 (take 필수)
    let range: Vec<i32> = Range::from(0, 3).take(7).collect();
    println!("0부터 3씩 7개: {:?}", range);

    let even_range: Vec<i32> = Range::from(0, 1)
        .filter(|x| x % 2 == 0)
        .take(5)
        .collect();
    println!("짝수 5개: {:?}", even_range);

    // ── 4. 표준 이터레이터들 ──────────────────────────────────
    println!("\n── 4. 표준 이터레이터 소비자 ──");

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 집계 소비자
    println!("sum:    {}", nums.iter().sum::<i32>());
    println!("product:{}", nums.iter().product::<i32>());
    println!("count:  {}", nums.iter().count());
    println!("max:    {:?}", nums.iter().max());
    println!("min:    {:?}", nums.iter().min());

    // 검색 소비자
    println!("find(>5): {:?}", nums.iter().find(|&&x| x > 5));
    println!("position(==5): {:?}", nums.iter().position(|&x| x == 5));
    println!("any(>9): {}", nums.iter().any(|&x| x > 9));
    println!("all(>0): {}", nums.iter().all(|&x| x > 0));

    // last와 nth
    println!("last: {:?}", nums.iter().last());
    println!("nth(4): {:?}", nums.iter().nth(4));  // 인덱스 4 = 5번째

    // ── 5. 배열과 슬라이스의 이터레이터 ──────────────────────
    println!("\n── 5. 배열/슬라이스 이터레이터 ──");

    let arr = [5, 3, 8, 1, 9, 2, 7];

    // 정렬 없이 최솟값 인덱스 찾기
    let min_idx = arr.iter()
        .enumerate()
        .min_by_key(|(_, &v)| v)
        .map(|(i, _)| i);
    println!("최솟값 인덱스: {:?}", min_idx);

    // 이진 검색 (정렬 후)
    let mut sorted = arr;
    sorted.sort();
    match sorted.binary_search(&7) {
        Ok(idx) => println!("7 발견, 인덱스: {}", idx),
        Err(idx) => println!("7 없음, 삽입 위치: {}", idx),
    }

    // ── 6. HashMap 이터레이터 ─────────────────────────────────
    println!("\n── 6. HashMap 이터레이터 ──");

    let mut scores = std::collections::HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    scores.insert("Dave", 78);

    // 이터레이션 (순서 불정)
    for (name, score) in &scores {
        println!("  {} → {}", name, score);
    }

    // 정렬된 이터레이션
    let mut sorted_scores: Vec<_> = scores.iter().collect();
    sorted_scores.sort_by_key(|(name, _)| *name);

    println!("이름순:");
    for (name, score) in &sorted_scores {
        println!("  {} → {}", name, score);
    }

    // 평균
    let avg: f64 = scores.values().map(|&v| v as f64).sum::<f64>()
        / scores.len() as f64;
    println!("평균: {:.1}", avg);
}
