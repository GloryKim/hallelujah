// 예제 07: 이터레이터 어댑터 완전 가이드
// 실행: cargo run --example 07_iterator_adapters

use std::collections::HashMap;

fn main() {
    println!("=== 이터레이터 어댑터 완전 가이드 ===\n");

    let data: Vec<i32> = (1..=10).collect();

    // ── 1. map — 변환 ────────────────────────────────────────
    println!("── 1. map (변환) ──");

    let doubled: Vec<i32> = data.iter().map(|&x| x * 2).collect();
    println!("두 배:   {:?}", doubled);

    let squared: Vec<i32> = data.iter().map(|&x| x * x).collect();
    println!("제곱:    {:?}", squared);

    let strings: Vec<String> = data.iter().map(|x| x.to_string()).collect();
    println!("문자열:  {:?}", strings);

    // 구조체로 변환
    #[derive(Debug)]
    struct Item { id: i32, value: String }

    let items: Vec<Item> = data.iter()
        .map(|&x| Item { id: x, value: format!("item_{}", x) })
        .collect();
    println!("구조체:  {:?}", &items[..3]);

    // ── 2. filter — 필터링 ───────────────────────────────────
    println!("\n── 2. filter (필터링) ──");

    let evens: Vec<&i32> = data.iter().filter(|&&x| x % 2 == 0).collect();
    println!("짝수:    {:?}", evens);

    let large: Vec<&i32> = data.iter().filter(|&&x| x > 5).collect();
    println!("5 초과:  {:?}", large);

    // 문자열 필터
    let words = vec!["Rust", "Python", "Java", "Go", "Ruby", "C++"];
    let long_words: Vec<&&str> = words.iter().filter(|w| w.len() > 3).collect();
    println!("4자 이상: {:?}", long_words);

    // ── 3. filter_map — 필터 + 변환 ──────────────────────────
    println!("\n── 3. filter_map (필터 + 변환) ──");

    let mixed = vec!["1", "두", "3", "넷", "5", "6", "일곱"];
    let numbers: Vec<i32> = mixed.iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("파싱 성공: {:?}", numbers);

    // Option 컬렉션 처리
    let options: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    let values: Vec<i32> = options.into_iter().filter_map(|x| x).collect();
    println!("Some 값들: {:?}", values);

    // ── 4. flat_map — 중첩 이터레이터 평탄화 ────────────────
    println!("\n── 4. flat_map (평탄화) ──");

    let sentences = vec!["Hello World", "Rust is awesome", "Flat Map example"];
    let words_flat: Vec<&str> = sentences.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("단어들: {:?}", words_flat);

    let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flatten: {:?}", flat);

    // 범위 생성
    let ranges: Vec<i32> = (1..=5)
        .flat_map(|x| (1..=x))
        .collect();
    println!("중첩 범위: {:?}", ranges);  // [1, 1,2, 1,2,3, ...]

    // ── 5. take / skip ────────────────────────────────────────
    println!("\n── 5. take / skip ──");

    let first5: Vec<i32> = data.iter().copied().take(5).collect();
    let after5: Vec<i32> = data.iter().copied().skip(5).collect();
    let middle: Vec<i32> = data.iter().copied().skip(2).take(5).collect();

    println!("처음 5: {:?}", first5);
    println!("5 이후: {:?}", after5);
    println!("중간 5: {:?}", middle);

    // take_while / skip_while
    let take_small: Vec<i32> = data.iter().copied().take_while(|&x| x < 5).collect();
    let skip_small: Vec<i32> = data.iter().copied().skip_while(|&x| x < 5).collect();

    println!("5 미만까지: {:?}", take_small);
    println!("5 이상부터: {:?}", skip_small);

    // ── 6. enumerate ──────────────────────────────────────────
    println!("\n── 6. enumerate (인덱스 + 값) ──");

    let fruits = ["사과", "바나나", "체리", "포도"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  [{}] {}", i, fruit);
    }

    // 1부터 시작하는 번호
    for (n, fruit) in fruits.iter().enumerate().map(|(i, x)| (i + 1, x)) {
        println!("  {}번째: {}", n, fruit);
    }

    // ── 7. zip ────────────────────────────────────────────────
    println!("\n── 7. zip (두 이터레이터 묶기) ──");

    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];
    let grades = vec!['A', 'B', 'A'];

    let combined: Vec<(&str, i32)> = names.iter()
        .zip(scores.iter())
        .map(|(&n, &s)| (n, s))
        .collect();
    println!("zip 결과: {:?}", combined);

    // 세 이터레이터 zip
    let three_zipped: Vec<_> = names.iter()
        .zip(scores.iter())
        .zip(grades.iter())
        .map(|((n, s), g)| format!("{}: {} ({})", n, s, g))
        .collect();
    println!("세 개 zip: {:?}", three_zipped);

    // ── 8. chain ──────────────────────────────────────────────
    println!("\n── 8. chain (이어 붙이기) ──");

    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let c = vec![7, 8, 9];

    let chained: Vec<i32> = a.iter()
        .chain(b.iter())
        .chain(c.iter())
        .copied()
        .collect();
    println!("chain: {:?}", chained);

    // ── 9. peekable ───────────────────────────────────────────
    println!("\n── 9. peekable (미리 보기) ──");

    let mut iter = vec![1, 2, 3, 4, 5].into_iter().peekable();

    while let Some(&next) = iter.peek() {
        if next % 2 == 0 {
            iter.next();  // 짝수는 건너뜀
        } else {
            println!("홀수 발견: {}", iter.next().unwrap());
        }
    }

    // ── 10. scan ──────────────────────────────────────────────
    println!("\n── 10. scan (상태 있는 변환) ──");

    // 누적 합
    let running_sum: Vec<i32> = data.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("누적 합: {:?}", running_sum);

    // 누적 최댓값
    let running_max: Vec<i32> = data.iter().rev()
        .scan(i32::MIN, |max, &x| {
            if x > *max { *max = x; }
            Some(*max)
        })
        .collect();
    println!("역순 누적 최대: {:?}", running_max);

    // ── 11. 복잡한 체인 ───────────────────────────────────────
    println!("\n── 11. 복잡한 이터레이터 체인 ──");

    let text = "the quick brown fox jumps over the lazy dog the end";

    // 단어 빈도 → 상위 3개
    let word_freq: HashMap<&str, usize> = text.split_whitespace()
        .fold(HashMap::new(), |mut map, w| {
            *map.entry(w).or_insert(0) += 1;
            map
        });

    let mut freq_vec: Vec<_> = word_freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    println!("상위 3개 단어:");
    for (word, count) in freq_vec.iter().take(3) {
        println!("  '{}': {}번", word, count);
    }

    // 숫자 파이프라인
    let result: i32 = (1..=100)
        .filter(|x| x % 3 == 0 || x % 5 == 0)  // 3 또는 5의 배수
        .map(|x| x * x)                           // 제곱
        .filter(|x| x % 2 == 0)                  // 짝수만
        .take(5)                                   // 5개만
        .sum();
    println!("\n3/5 배수 제곱 중 짝수 5개 합: {}", result);

    // ── 12. collect 다양한 타입으로 ──────────────────────────
    println!("\n── 12. collect 타겟 타입 ──");

    let v: Vec<i32> = (1..=5).collect();
    println!("Vec: {:?}", v);

    let set: std::collections::HashSet<i32> = (1..=5).chain(3..=7).collect();
    println!("HashSet: {:?}", set);

    let btree: std::collections::BTreeSet<i32> = (1..=5).chain(3..=7).collect();
    println!("BTreeSet: {:?}", btree);  // 정렬됨

    let map: HashMap<i32, i32> = (1..=5).map(|x| (x, x * x)).collect();
    println!("HashMap: {:?}", map);

    let string: String = vec!['h', 'e', 'l', 'l', 'o'].into_iter().collect();
    println!("String: {}", string);

    // ── 13. 지연 평가 (lazy evaluation) ──────────────────────
    println!("\n── 13. 지연 평가 확인 ──");

    let count = std::cell::Cell::new(0);
    let lazy_chain = (0..1000)
        .map(|x| {
            count.set(count.get() + 1);
            x * x
        })
        .filter(|x| x % 7 == 0)
        .take(5);  // 아직 아무것도 실행 안 됨!

    println!("체인 생성 후 map 호출 횟수: {}", count.get());
    let result: Vec<i32> = lazy_chain.collect();  // 여기서 실행
    println!("collect 후 map 호출 횟수: {}", count.get());
    println!("결과: {:?}", result);
}
