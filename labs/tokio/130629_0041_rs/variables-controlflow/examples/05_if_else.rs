// 예제 05: if / else — 조건 분기 완전 가이드
// 실행: cargo run --example 05_if_else

fn classify_score(score: u32) -> &'static str {
    if score >= 90 {
        "A (수)"
    } else if score >= 80 {
        "B (우)"
    } else if score >= 70 {
        "C (미)"
    } else if score >= 60 {
        "D (양)"
    } else {
        "F (불)"
    }
}

fn bmi_category(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "저체중"
    } else if bmi < 23.0 {
        "정상"
    } else if bmi < 25.0 {
        "과체중"
    } else if bmi < 30.0 {
        "비만 1단계"
    } else {
        "비만 2단계"
    }
}

fn fizzbuzz(n: u32) -> String {
    if n % 15 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}

fn main() {
    println!("=== if / else 완전 가이드 ===\n");

    // ── 기본 if / else ────────────────────────────────────────
    println!("── 1. 기본 if / else ──");

    let temperature = 28;

    if temperature > 30 {
        println!("더워요! ({}°C)", temperature);
    } else if temperature > 20 {
        println!("쾌적해요! ({}°C)", temperature);
    } else if temperature > 10 {
        println!("서늘해요. ({}°C)", temperature);
    } else {
        println!("추워요! ({}°C)", temperature);
    }

    // 조건은 반드시 bool (정수 불가)
    let flag = 1;
    if flag != 0 {  // C와 달리 if flag {} 불가
        println!("flag는 0이 아님");
    }

    // ── if 표현식 ─────────────────────────────────────────────
    println!("\n── 2. if 표현식 (값 반환) ──");

    let score = 85;
    let grade = if score >= 60 { "합격" } else { "불합격" };
    println!("점수 {}: {}", score, grade);

    // 더 복잡한 표현식
    let x = 10;
    let abs_x = if x >= 0 { x } else { -x };
    println!("|{}| = {}", x, abs_x);

    // 다단계 if 표현식
    let level = if score >= 90 {
        "골드"
    } else if score >= 70 {
        "실버"
    } else {
        "브론즈"
    };
    println!("레벨: {}", level);

    // 주의: 모든 분기의 타입이 같아야 함
    let num = 5;
    let result: i32 = if num > 0 {
        println!("양수 분기 실행");
        num * 10
    } else {
        println!("음수/0 분기 실행");
        -num
    };
    println!("결과: {}", result);

    // ── if let ────────────────────────────────────────────────
    println!("\n── 3. if let (패턴 매칭 축약) ──");

    let maybe: Option<i32> = Some(42);

    // match 버전 (장황함)
    match maybe {
        Some(n) => println!("match Some: {}", n),
        None => println!("match None"),
    }

    // if let 버전 (간결함)
    if let Some(n) = maybe {
        println!("if let Some: {}", n);
    } else {
        println!("없음");
    }

    // 타입 검사와 함께
    let values: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    let mut sum = 0;
    for val in &values {
        if let Some(n) = val {
            sum += n;
        }
    }
    println!("Some 값들의 합: {}", sum);

    // Result와 함께
    let parsed: Result<i32, _> = "42".parse();
    if let Ok(n) = parsed {
        println!("파싱 성공: {}", n);
    }

    let bad: Result<i32, _> = "abc".parse::<i32>();
    if let Err(e) = bad {
        println!("파싱 실패: {}", e);
    }

    // ── let else (1.65+) ──────────────────────────────────────
    println!("\n── 4. let else (조기 반환 패턴) ──");

    fn parse_positive(s: &str) -> Option<u32> {
        let Ok(n) = s.parse::<i32>() else {
            println!("  '{}' 파싱 실패", s);
            return None;
        };

        let Ok(positive) = u32::try_from(n) else {
            println!("  {} 는 음수", n);
            return None;
        };

        Some(positive)
    }

    println!("{:?}", parse_positive("42"));
    println!("{:?}", parse_positive("abc"));
    println!("{:?}", parse_positive("-5"));

    // ── 중첩 if / 복잡한 조건 ────────────────────────────────
    println!("\n── 5. 복잡한 조건 ──");

    let age = 25;
    let has_license = true;
    let blood_alcohol = 0.0f64;

    // 논리 연산자
    if age >= 18 && has_license && blood_alcohol < 0.05 {
        println!("운전 가능");
    } else if !has_license {
        println!("면허 없음");
    } else if age < 18 {
        println!("미성년자");
    } else {
        println!("음주 운전 금지");
    }

    // 복합 조건에서 단락 평가 활용
    let v: Vec<i32> = vec![1, 2, 3];
    let target = 5;
    if !v.is_empty() && v[0] < target {
        println!("첫 요소 {} < {}", v[0], target);
    }

    // ── 실용 예제들 ───────────────────────────────────────────
    println!("\n── 6. 실용 예제들 ──");

    // 성적 분류
    for score in [95, 85, 75, 65, 45] {
        println!("  {} → {}", score, classify_score(score));
    }

    // BMI 계산
    println!();
    let weights = [50.0, 65.0, 75.0, 90.0, 110.0];
    let height = 1.70;
    for weight in weights {
        let bmi = weight / (height * height);
        println!("  {}kg → BMI {:.1} → {}", weight, bmi, bmi_category(bmi));
    }

    // FizzBuzz
    println!("\nFizzBuzz (1~20):");
    let result: Vec<String> = (1..=20).map(fizzbuzz).collect();
    println!("{}", result.join(", "));

    // ── 삼항 연산자 대체 ──────────────────────────────────────
    println!("\n── 7. 삼항 연산자 스타일 ──");

    let n = 7;

    // 다른 언어: n > 0 ? "양수" : "음수"
    // Rust:
    let sign = if n > 0 { "양수" } else if n < 0 { "음수" } else { "0" };
    println!("{} 은 {}", n, sign);

    // 함수 호출에서도 사용
    fn repeat(s: &str, count: usize) -> String {
        s.repeat(count)
    }

    let border = repeat("─", if n > 5 { 20 } else { 10 });
    println!("{}", border);
}
