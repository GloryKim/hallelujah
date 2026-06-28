// 예제 10: fold, scan, reduce — 누적 연산의 모든 것
// 실행: cargo run --example 10_fold_and_scan

use std::collections::HashMap;

fn main() {
    println!("=== fold / scan / reduce 완전 가이드 ===\n");

    let data: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    // ── 1. fold 기초 ──────────────────────────────────────────
    println!("── 1. fold 기초 ──");
    println!(
        r#"
  fold(초기값, |누산기, 요소| -> 누산기) → 최종값

  동작 방식:
    acc = 초기값
    for elem in iter:
        acc = f(acc, elem)
    return acc
"#
    );

    // 합
    let sum = data.iter().fold(0, |acc, &x| acc + x);
    println!("합(fold):     {}", sum);
    println!("합(sum):      {}", data.iter().sum::<i32>());  // 동일

    // 곱
    let product = (1..=5).fold(1, |acc, x: i32| acc * x);
    println!("1~5 곱:       {}", product);  // 120 = 5!

    // 최댓값
    let max = data.iter().fold(i32::MIN, |acc, &x| acc.max(x));
    println!("최댓값:       {}", max);

    // 카운트
    let count = data.iter().fold(0usize, |acc, _| acc + 1);
    println!("카운트:       {}", count);

    // 조건부 카운트
    let odd_count = data.iter().fold(0, |acc, &x| if x % 2 != 0 { acc + 1 } else { acc });
    println!("홀수 카운트:  {}", odd_count);

    // ── 2. fold로 컬렉션 구축 ─────────────────────────────────
    println!("\n── 2. fold로 컬렉션 구축 ──");

    // Vec 구축
    let evens: Vec<i32> = data.iter()
        .fold(Vec::new(), |mut acc, &x| {
            if x % 2 == 0 { acc.push(x); }
            acc
        });
    println!("짝수 Vec:     {:?}", evens);

    // HashMap 구축 (단어 빈도)
    let text = "apple banana apple cherry banana apple cherry apple";
    let freq: HashMap<&str, usize> = text.split_whitespace()
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        });
    let mut freq_vec: Vec<_> = freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("단어 빈도:    {:?}", freq_vec);

    // String 조합
    let csv = data.iter()
        .fold(String::new(), |mut s, x| {
            if !s.is_empty() { s.push(','); }
            s.push_str(&x.to_string());
            s
        });
    println!("CSV:          {}", csv);

    // 더 관용적:
    let csv2 = data.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("CSV2:         {}", csv2);

    // ── 3. fold로 알고리즘 구현 ───────────────────────────────
    println!("\n── 3. fold로 알고리즘 구현 ──");

    // flatten
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat: Vec<i32> = nested.into_iter()
        .fold(Vec::new(), |mut acc, v| {
            acc.extend(v);
            acc
        });
    println!("flatten:      {:?}", flat);

    // 역순
    let reversed: Vec<i32> = data.iter()
        .fold(Vec::new(), |mut acc, &x| {
            acc.insert(0, x);
            acc
        });
    println!("reversed:     {:?}", reversed);

    // 그룹화 (짝수/홀수)
    let (evens2, odds): (Vec<i32>, Vec<i32>) = data.iter()
        .fold((Vec::new(), Vec::new()), |(mut evs, mut odds), &x| {
            if x % 2 == 0 { evs.push(x) } else { odds.push(x) };
            (evs, odds)
        });
    println!("짝수:         {:?}", evens2);
    println!("홀수:         {:?}", odds);

    // 평균 (합과 개수 동시)
    let (total, cnt) = data.iter().fold((0i64, 0usize), |(s, c), &x| (s + x as i64, c + 1));
    println!("평균:         {:.2}", total as f64 / cnt as f64);

    // ── 4. reduce ─────────────────────────────────────────────
    println!("\n── 4. reduce (초기값 없는 fold) ──");

    // reduce: 첫 요소를 초기값으로 사용
    let max = data.iter().copied().reduce(|a, b| if a > b { a } else { b });
    let min = data.iter().copied().reduce(|a, b| if a < b { a } else { b });
    let sum2 = data.iter().copied().reduce(|a, b| a + b);

    println!("max (reduce): {:?}", max);
    println!("min (reduce): {:?}", min);
    println!("sum (reduce): {:?}", sum2);

    // 빈 이터레이터에서 reduce → None
    let empty: Vec<i32> = vec![];
    println!("reduce(빈):   {:?}", empty.iter().copied().reduce(|a, b| a + b));

    // ── 5. scan ───────────────────────────────────────────────
    println!("\n── 5. scan (중간 상태를 스트림으로) ──");
    println!(
        r#"
  scan과 fold의 차이:
    fold:  중간값 불필요, 최종값만 반환
    scan:  모든 중간 누산기 값을 이터레이터로 반환
"#
    );

    let numbers = vec![1, 2, 3, 4, 5];

    // 누적 합 (running sum)
    let running_sum: Vec<i32> = numbers.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("누적 합:      {:?}", running_sum);  // [1, 3, 6, 10, 15]

    // 누적 곱
    let running_product: Vec<i32> = numbers.iter()
        .scan(1, |acc, &x| {
            *acc *= x;
            Some(*acc)
        })
        .collect();
    println!("누적 곱:      {:?}", running_product);  // [1, 2, 6, 24, 120]

    // 누적 최대
    let running_max: Vec<i32> = data.iter()
        .scan(i32::MIN, |max, &x| {
            if x > *max { *max = x; }
            Some(*max)
        })
        .collect();
    println!("누적 최대:    {:?}", running_max);

    // scan으로 중간 종료 (None 반환)
    let early_stop: Vec<i32> = numbers.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            if *acc < 10 { Some(*acc) } else { None }  // None → 종료
        })
        .collect();
    println!("10 미만까지:  {:?}", early_stop);

    // ── 6. 복잡한 fold 예제 ───────────────────────────────────
    println!("\n── 6. 복잡한 fold 예제 ──");

    // RLE (Run-Length Encoding)
    let input = vec![1, 1, 1, 2, 2, 3, 1, 1];
    let rle: Vec<(i32, usize)> = input.iter()
        .fold(Vec::new(), |mut acc, &x| {
            match acc.last_mut() {
                Some(last) if last.0 == x => last.1 += 1,
                _ => acc.push((x, 1)),
            }
            acc
        });
    println!("RLE {:?} → {:?}", input, rle);

    // 역 RLE 디코딩
    let decoded: Vec<i32> = rle.iter()
        .fold(Vec::new(), |mut acc, &(val, count)| {
            acc.extend(std::iter::repeat(val).take(count));
            acc
        });
    println!("RLE 디코딩 → {:?}", decoded);

    // 최장 증가 부분 수열 길이 (O(n²) DP with fold)
    let seq = vec![3, 10, 2, 1, 20];
    let lis_len = seq.iter().enumerate()
        .fold(vec![1usize; seq.len()], |mut dp, (i, _)| {
            for j in 0..i {
                if seq[j] < seq[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            dp
        })
        .into_iter()
        .max()
        .unwrap_or(0);
    println!("LIS 길이 {:?}: {}", seq, lis_len);

    // ── 7. try_fold / try_for_each ────────────────────────────
    println!("\n── 7. try_fold (실패 가능한 fold) ──");

    let strings = vec!["1", "2", "3", "4", "5"];
    let sum_result: Result<i32, _> = strings.iter()
        .try_fold(0, |acc, s| {
            s.parse::<i32>().map(|n| acc + n)
        });
    println!("파싱 합: {:?}", sum_result);

    let bad_strings = vec!["1", "2", "bad", "4"];
    let err_result: Result<i32, _> = bad_strings.iter()
        .try_fold(0, |acc, s| {
            s.parse::<i32>().map(|n| acc + n)
        });
    println!("파싱 실패: {:?}", err_result);

    // ── 8. 루프를 fold로 재작성 ───────────────────────────────
    println!("\n── 8. 루프 → fold 변환 ──");

    // 원래 코드:
    let mut result_loop = 0;
    for &x in &data {
        if x > 3 {
            result_loop += x * x;
        }
    }

    // fold 버전:
    let result_fold = data.iter()
        .filter(|&&x| x > 3)
        .fold(0, |acc, &x| acc + x * x);

    println!("루프 결과: {}", result_loop);
    println!("fold 결과: {}", result_fold);
    assert_eq!(result_loop, result_fold);
    println!("일치 ✓");
}
