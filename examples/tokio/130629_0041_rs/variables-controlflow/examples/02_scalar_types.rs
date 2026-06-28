// 예제 02: 스칼라 타입 완전 가이드
// 실행: cargo run --example 02_scalar_types

fn section(title: &str) {
    println!("\n{}", "─".repeat(55));
    println!("  {}", title);
    println!("{}", "─".repeat(55));
}

fn main() {
    section("1. 정수 타입 — 크기와 범위");

    println!("{:<8} {:>6} {:>25} {:>25}", "타입", "비트", "최솟값", "최댓값");
    println!("{}", "─".repeat(70));
    println!("{:<8} {:>6} {:>25} {:>25}", "i8",    8,   i8::MIN,    i8::MAX);
    println!("{:<8} {:>6} {:>25} {:>25}", "i16",  16,  i16::MIN,   i16::MAX);
    println!("{:<8} {:>6} {:>25} {:>25}", "i32",  32,  i32::MIN,   i32::MAX);
    println!("{:<8} {:>6} {:>25} {:>25}", "i64",  64,  i64::MIN,   i64::MAX);
    println!("{:<8} {:>6} {:>25} {:>25}", "u8",    8,   u8::MIN,    u8::MAX);
    println!("{:<8} {:>6} {:>25} {:>25}", "u16",  16,  u16::MIN,   u16::MAX);
    println!("{:<8} {:>6} {:>25} {:>25}", "u32",  32,  u32::MIN,   u32::MAX);
    println!("{:<8} {:>6} {:>25} {:>25}", "u64",  64,  u64::MIN,   u64::MAX);
    println!("{:<8} {:>6} {:>25} {:>25}", "usize", std::mem::size_of::<usize>()*8,
             usize::MIN, usize::MAX);

    section("2. 정수 리터럴 표기법");

    let decimal     = 1_000_000;     // 밑줄 = 가독성 구분자
    let hex         = 0xFF_AB;
    let octal       = 0o777;
    let binary      = 0b1010_1010;
    let byte: u8    = b'A';          // u8만 가능

    println!("10진수:  {} = {}", "1_000_000", decimal);
    println!("16진수:  {} = {}", "0xFF_AB",   hex);
    println!(" 8진수:  {} = {}", "0o777",     octal);
    println!(" 2진수:  {} = {}", "0b1010_1010", binary);
    println!("  byte:  {} = '{}'", "b'A'",    byte as char);

    // 타입 접미사
    let typed = 42u64;
    let typed2 = 3.14f32;
    println!("타입 접미사: {}u64, {}f32", typed, typed2);

    section("3. 정수 오버플로 처리");

    let max_u8: u8 = 255;

    // 1) wrapping — 넘으면 wrap-around
    let w = max_u8.wrapping_add(1);
    println!("255u8.wrapping_add(1) = {} (순환)", w);

    let w2 = 0u8.wrapping_sub(1);
    println!("0u8.wrapping_sub(1)   = {} (순환)", w2);

    // 2) checked — 오버플로 시 None
    let c = max_u8.checked_add(1);
    let c2 = max_u8.checked_add(0);
    println!("255u8.checked_add(1)  = {:?}", c);   // None
    println!("255u8.checked_add(0)  = {:?}", c2);  // Some(255)

    // 3) saturating — 오버플로 시 최댓값/최솟값
    let s = max_u8.saturating_add(100);
    let s2 = 0u8.saturating_sub(50);
    println!("255u8.saturating_add(100) = {} (포화)", s);
    println!("0u8.saturating_sub(50)    = {} (포화)", s2);

    // 4) overflowing — (결과, 오버플로여부) 반환
    let (val, did_overflow) = max_u8.overflowing_add(1);
    println!("255u8.overflowing_add(1)  = ({}, {})", val, did_overflow);

    section("4. 정수 연산");

    let a: i32 = 17;
    let b: i32 = 5;
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {} (정수 나눗셈, 버림)", a, b, a / b);
    println!("{} % {} = {} (나머지)", a, b, a % b);

    // 음수 나머지
    println!("-17 % 5 = {} (부호는 피제수를 따름)", -17 % 5);

    // 비트 연산
    let x: u8 = 0b1100;
    let y: u8 = 0b1010;
    println!("\n비트 연산 ({:04b}, {:04b}):", x, y);
    println!("AND: {:04b}", x & y);
    println!(" OR: {:04b}", x | y);
    println!("XOR: {:04b}", x ^ y);
    println!("NOT: {:08b}", !x);
    println!("SHL 1: {:04b}", x << 1);
    println!("SHR 1: {:04b}", x >> 1);

    // 수학 함수
    let n: i32 = -42;
    println!("\n수학 함수:");
    println!("abs(-42)     = {}", n.abs());
    println!("pow(2, 10)   = {}", 2i32.pow(10));
    println!("sqrt(144.0)  = {}", (144.0f64).sqrt());
    println!("min(3,5)     = {}", 3i32.min(5));
    println!("max(3,5)     = {}", 3i32.max(5));
    println!("clamp(15,0,10) = {}", 15i32.clamp(0, 10));

    section("5. 부동소수점 타입");

    let f32_val: f32 = 3.14_f32;
    let f64_val: f64 = 3.141_592_653_589_793;

    println!("f32 (7자리 정밀도):  {:.10}", f32_val);
    println!("f64 (15자리 정밀도): {:.15}", f64_val);

    // 특수 값
    println!("\n부동소수점 특수 값:");
    println!("INFINITY:     {}", f64::INFINITY);
    println!("NEG_INFINITY: {}", f64::NEG_INFINITY);
    println!("NAN:          {}", f64::NAN);
    println!("EPSILON:      {}", f64::EPSILON);
    println!("MAX:          {}", f64::MAX);
    println!("MIN_POSITIVE: {}", f64::MIN_POSITIVE);

    // NaN 특성 — NaN은 자기 자신과도 같지 않음 (IEEE 754 표준)
    // invalid_nan_comparisons: 교육용으로 의도된 코드 (NaN의 특이한 동작 시연)
    #[allow(invalid_nan_comparisons)]
    {
        println!("\nNaN 특성:");
        println!("NAN == NAN: {}", f64::NAN == f64::NAN);    // false (NaN의 특이성!)
        println!("NAN != NAN: {}", f64::NAN != f64::NAN);    // true
    }
    println!("NAN.is_nan(): {}", f64::NAN.is_nan());          // true
    println!("INFINITY.is_infinite(): {}", f64::INFINITY.is_infinite());
    println!("1.0f64.is_finite(): {}", 1.0f64.is_finite());

    // 부동소수점 함수
    let pi = std::f64::consts::PI;
    println!("\n수학 함수 (f64):");
    println!("sin(π/2) = {:.6}", (pi / 2.0).sin());
    println!("cos(0)   = {:.6}", 0.0f64.cos());
    println!("sqrt(2)  = {:.10}", 2.0f64.sqrt());
    println!("exp(1)   = {:.10}", 1.0f64.exp());  // e
    println!("ln(e)    = {:.10}", std::f64::consts::E.ln());
    println!("log2(8)  = {:.6}", 8.0f64.log2());
    println!("floor(3.7) = {}", 3.7f64.floor());
    println!("ceil(3.2)  = {}", 3.2f64.ceil());
    println!("round(3.5) = {}", 3.5f64.round());
    println!("trunc(3.9) = {}", 3.9f64.trunc());
    println!("fract(3.9) = {}", 3.9f64.fract());  // 0.9 (소수 부분)

    section("6. bool 타입");

    let t = true;
    let f = false;

    println!("true && false = {}", t && f);
    println!("true || false = {}", t || f);
    println!("!true         = {}", !t);

    // 단락 평가 (short-circuit evaluation)
    let x = 0;
    let safe = x != 0 && (10 / x > 0);  // x가 0이면 오른쪽 평가 안 함
    println!("단락 평가 (0 나눗셈 없음): {}", safe);

    // bool → 정수
    println!("true as i32  = {}", true as i32);
    println!("false as i32 = {}", false as i32);

    // 비교 연산자
    println!("\n비교 연산자:");
    println!("5 == 5: {}", 5 == 5);
    println!("5 != 6: {}", 5 != 6);
    println!("3 < 5:  {}", 3 < 5);
    println!("5 >= 5: {}", 5 >= 5);

    section("7. char 타입 (유니코드 스칼라값)");

    let ascii = 'A';
    let latin = 'é';
    let hangul = '한';
    let emoji = '🦀';
    let null = '\0';

    println!("'A'  = U+{:04X}, 크기 {} bytes", ascii as u32, std::mem::size_of::<char>());
    println!("'é'  = U+{:04X}", latin as u32);
    println!("'한' = U+{:04X}", hangul as u32);
    println!("'🦀' = U+{:04X}", emoji as u32);
    println!("'\\0'= U+{:04X}", null as u32);

    // char 메서드
    println!("\nchar 메서드:");
    println!("'A'.is_alphabetic()  = {}", 'A'.is_alphabetic());
    println!("'a'.is_lowercase()   = {}", 'a'.is_lowercase());
    println!("'A'.is_uppercase()   = {}", 'A'.is_uppercase());
    println!("'1'.is_numeric()     = {}", '1'.is_numeric());
    println!("'1'.is_alphanumeric()= {}", '1'.is_alphanumeric());
    println!("' '.is_whitespace()  = {}", ' '.is_whitespace());
    println!("'a'.to_uppercase()   = {}", 'a'.to_uppercase().next().unwrap());

    // char → 숫자 변환
    let digit = '7';
    println!("\n'7'.to_digit(10) = {:?}", digit.to_digit(10));
    println!("'F'.to_digit(16) = {:?}", 'F'.to_digit(16));

    // 숫자 → char
    let c = char::from_u32(65).unwrap();  // 'A'
    println!("char::from_u32(65) = '{}'", c);

    section("8. 타입 변환 (as 캐스팅)");

    let i: i32 = 300;
    let f: f64 = i as f64;
    let u: u8 = i as u8;     // 오버플로! 300 - 256 = 44
    let b: bool = i != 0;

    println!("i32(300) as f64 = {}", f);
    println!("i32(300) as u8  = {} (오버플로 truncate)", u);
    println!("i32(300) != 0   = {}", b);

    let c = 'A';
    println!("'A' as u8  = {}", c as u8);
    println!("'A' as i32 = {}", c as i32);

    let neg: i32 = -1;
    println!("i32(-1) as u32 = {} (재해석)", neg as u32);  // u32::MAX
}
