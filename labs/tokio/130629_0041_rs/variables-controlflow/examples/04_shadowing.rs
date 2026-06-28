// 예제 04: 섀도잉 (Shadowing) 완전 가이드
// 실행: cargo run --example 04_shadowing

fn main() {
    println!("=== 섀도잉(Shadowing) 완전 가이드 ===\n");

    // ── 기본 섀도잉 ───────────────────────────────────────────
    println!("── 1. 기본 섀도잉 ──");

    let x = 5;
    println!("초기 x = {}", x);         // 5

    let x = x + 1;
    println!("x = x + 1 → x = {}", x);  // 6

    let x = x * 2;
    println!("x = x * 2 → x = {}", x);  // 12

    // 블록 내 섀도잉 (스코프 종료 시 복원)
    {
        let x = x + 100;
        println!("블록 내 x = {}", x);   // 112
    }
    println!("블록 밖 x = {}", x);       // 12 (복원됨)

    // ── 타입 변경 섀도잉 ──────────────────────────────────────
    println!("\n── 2. 타입 변경 섀도잉 (핵심 용도) ──");

    // 파싱 패턴: 이름 재사용으로 코드 명확성 향상
    let input = "  42  ";                        // &str
    println!("입력 (문자열): {:?}", input);

    let input = input.trim();                    // 공백 제거한 &str
    println!("트림 후: {:?}", input);

    let input: i32 = input.parse().unwrap();     // i32로 변환
    println!("파싱 후: {} (i32)", input);

    let input = input * 2;                       // 연산
    println!("*2 후: {}", input);

    // mut으로는 타입 변경 불가 — 이것이 섀도잉의 핵심 차이점
    // let mut s = "hello";
    // s = s.len();  // 오류! expected &str, found usize

    // ── 섀도잉 vs mut 비교 ────────────────────────────────────
    println!("\n── 3. 섀도잉 vs mut ──");

    // mut: 가변 변수, 같은 타입만, 재바인딩이 아님
    let mut counter = 0;
    counter += 1;
    counter += 1;
    counter += 1;
    println!("mut counter: {}", counter);  // 3

    // shadowing: 새 변수 생성, 타입 변경 가능, 이전 값 참조 가능
    let steps = 0;
    let steps = steps + 1;  // 새 steps (이전 steps 참조)
    let steps = steps + 1;
    let steps = steps + 1;
    println!("shadowed steps: {}", steps);  // 3

    // ── 실용 패턴들 ───────────────────────────────────────────
    println!("\n── 4. 실용 패턴 ──");

    // 패턴 1: 조건에 따라 타입 변환
    let raw = "3.14";
    let value: f64 = raw.parse().unwrap_or(0.0);
    let value = (value * 100.0).round() / 100.0;  // 소수점 2자리
    println!("처리된 값: {}", value);

    // 패턴 2: 불필요한 mut 제거 (Clippy가 권장)
    // 나쁜 예:
    let mut result = Vec::new();
    result.push(1);
    result.push(2);
    result.push(3);
    let result = result;  // mut 잠금
    println!("결과: {:?}", result);

    // 더 좋은 예:
    let config = {
        let mut temp = std::collections::HashMap::new();
        temp.insert("host", "localhost");
        temp.insert("port", "8080");
        temp  // 불변으로 반환
    };
    println!("설정: {:?}", config);

    // 패턴 3: 중간 계산 단계 명명
    let raw_data = "  hello world  ";
    let raw_data = raw_data.trim();
    let raw_data = raw_data.to_uppercase();
    let raw_data: Vec<&str> = raw_data.split_whitespace().collect();
    println!("처리 체인 결과: {:?}", raw_data);

    // 패턴 4: 단위 변환
    let price_won = 50000;               // 원화 (i32)
    let price_usd = price_won as f64 / 1350.0;
    let price_usd = (price_usd * 100.0).round() / 100.0;
    println!("{}원 = ${}", price_won, price_usd);

    // 패턴 5: Option 언래핑 후 재사용
    let maybe_number: Option<i32> = Some(42);
    if let Some(maybe_number) = maybe_number {  // 섀도잉으로 언래핑
        println!("값: {} (언래핑됨)", maybe_number);
    }

    // ── 스코프와 섀도잉 ───────────────────────────────────────
    println!("\n── 5. 스코프와 섀도잉 심화 ──");

    let msg = "외부";
    println!("진입 전: {}", msg);

    {
        let msg = "내부 1";
        println!("블록 1: {}", msg);

        {
            let msg = "내부 2";
            println!("블록 2: {}", msg);
        }

        println!("블록 1 복귀: {}", msg);  // "내부 1"
    }

    println!("종료 후: {}", msg);  // "외부"

    // ── 함수 파라미터 섀도잉 ──────────────────────────────────
    println!("\n── 6. 함수 내 섀도잉 ──");

    fn process(value: i32) -> String {
        let value = value.abs();          // 음수 → 양수
        let value = value.to_string();    // i32 → String
        let value = format!("값: {}", value);
        value
    }

    println!("{}", process(-42));
    println!("{}", process(100));
}
