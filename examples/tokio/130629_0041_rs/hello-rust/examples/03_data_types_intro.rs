// 예제 03: 기본 타입 미리보기 (Week 2에서 자세히 다룸)
// 실행: cargo run --example 03_data_types_intro

fn main() {
    println!("=== Rust 기본 타입 한눈에 보기 ===\n");

    // ── 정수 타입 ────────────────────────────────────────────────
    println!("── 정수 타입 ──");

    let a: i8 = 127;           // -128 ~ 127
    let b: i16 = 32767;        // -32768 ~ 32767
    let c: i32 = 2_147_483_647; // 약 ±21억 (기본 정수 타입)
    let d: i64 = 9_223_372_036_854_775_807;
    let e: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let f: isize = 100;        // 플랫폼에 따라 32 또는 64비트

    let g: u8 = 255;           // 0 ~ 255
    let h: u16 = 65535;
    let i: u32 = 4_294_967_295;
    let j: u64 = 18_446_744_073_709_551_615;
    let k: usize = 42;         // 메모리 주소 크기 (포인터와 동일)

    println!("i8  max: {}", a);
    println!("i16 max: {}", b);
    println!("i32 max: {}", c);
    println!("i64 max: {}", d);
    println!("i128 max: {}", e);
    println!("isize: {}", f);
    println!("u8  max: {}", g);
    println!("u32 max: {}", i);
    println!("usize: {}", k);

    // 타입별 최솟값/최댓값
    println!("\n타입 한계:");
    println!("i32: {} ~ {}", i32::MIN, i32::MAX);
    println!("u32: {} ~ {}", u32::MIN, u32::MAX);
    println!("i64: {} ~ {}", i64::MIN, i64::MAX);

    // 정수 리터럴 표기법
    let decimal = 1_000_000;   // 밑줄로 가독성 향상
    let hex = 0xFF;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';           // u8만 가능

    println!("\n리터럴 표기:");
    println!("10진수: {}", decimal);
    println!("16진수: {} ({})", hex, 0xFF_u32);
    println!(" 8진수: {}", octal);
    println!(" 2진수: {}", binary);
    println!("  byte: {} ('{}')", byte, byte as char);

    // ── 부동소수점 ───────────────────────────────────────────────
    println!("\n── 부동소수점 ──");

    let x: f32 = 3.14;
    let y: f64 = 3.141592653589793;  // 기본 타입

    println!("f32: {:.6}", x);
    println!("f64: {:.15}", y);
    println!("f64 epsilon: {}", f64::EPSILON);
    println!("f64 infinity: {}", f64::INFINITY);
    println!("f64 NaN: {}", f64::NAN);
    println!("NaN == NaN: {}", f64::NAN == f64::NAN);  // false!
    println!("is NaN: {}", f64::NAN.is_nan());           // true

    // 부동소수점 함정
    let a = 0.1_f64 + 0.2_f64;
    println!("\n0.1 + 0.2 = {:.20}", a);  // 정확히 0.3이 아님!
    println!("0.1 + 0.2 == 0.3: {}", (a - 0.3).abs() < f64::EPSILON);

    // ── 불리언 ───────────────────────────────────────────────────
    println!("\n── 불리언 ──");

    let t: bool = true;
    let f_val: bool = false;
    println!("true: {}, false: {}", t, f_val);
    println!("AND: {}, OR: {}, NOT: {}", t && f_val, t || f_val, !t);
    println!("bool 크기: {} 바이트", std::mem::size_of::<bool>());

    // ── 문자 ─────────────────────────────────────────────────────
    println!("\n── char (유니코드 스칼라값) ──");

    let ch: char = 'A';
    let korean: char = '한';
    let emoji: char = '🦀';

    println!("ASCII: {}", ch);
    println!("한국어: {}", korean);
    println!("이모지: {}", emoji);
    println!("char 크기: {} 바이트 (항상 4바이트)", std::mem::size_of::<char>());
    println!("'A' as u32 = {}", ch as u32);
    println!("'한' as u32 = {}", korean as u32);
    println!("char::MAX = U+{:X}", char::MAX as u32);

    // 문자 메서드
    println!("\n문자 메서드:");
    println!("'A'.is_alphabetic(): {}", 'A'.is_alphabetic());
    println!("'1'.is_numeric(): {}", '1'.is_numeric());
    println!("' '.is_whitespace(): {}", ' '.is_whitespace());
    println!("'a'.to_uppercase(): {}", 'a'.to_uppercase().next().unwrap());
    println!("'A'.to_lowercase(): {}", 'A'.to_lowercase().next().unwrap());

    // ── 타입 크기 요약 ───────────────────────────────────────────
    println!("\n── 타입 메모리 크기 ──");
    println!("bool:   {} byte", std::mem::size_of::<bool>());
    println!("i8:     {} byte", std::mem::size_of::<i8>());
    println!("i16:    {} bytes", std::mem::size_of::<i16>());
    println!("i32:    {} bytes", std::mem::size_of::<i32>());
    println!("i64:    {} bytes", std::mem::size_of::<i64>());
    println!("i128:   {} bytes", std::mem::size_of::<i128>());
    println!("isize:  {} bytes (이 시스템)", std::mem::size_of::<isize>());
    println!("f32:    {} bytes", std::mem::size_of::<f32>());
    println!("f64:    {} bytes", std::mem::size_of::<f64>());
    println!("char:   {} bytes", std::mem::size_of::<char>());
}
