// 예제 02: print 계열 매크로 + 표준 입력
// 실행: cargo run --example 02_print_variants

use std::io::{self, Write};

fn print_separator(title: &str) {
    let line = "─".repeat(50);
    println!("\n{}", line);
    println!("  {}", title);
    println!("{}", line);
}

fn main() {
    print_separator("1. println! vs print!");

    // println!: 출력 후 자동 개행
    println!("첫 번째 줄");
    println!("두 번째 줄");

    // print!: 개행 없음 — flush 필요할 수 있음
    print!("같은 ");
    print!("줄에 ");
    print!("출력됩니다");
    println!();  // 빈 println!으로 줄바꿈

    // 명시적 flush (버퍼 즉시 비우기)
    print!("즉시 출력: ");
    io::stdout().flush().unwrap();
    println!("완료");

    print_separator("2. eprintln! — 표준 에러(stderr)");

    // stderr는 stdout과 별도 스트림
    // 파이프라인에서 에러만 분리 가능: ./app 2>error.log
    eprintln!("경고: 이것은 stderr로 갑니다");
    eprintln!("에러 코드: {}", 404);

    // 일반 에러 패턴
    let result: Result<i32, &str> = Err("파일 없음");
    if let Err(e) = result {
        eprintln!("오류 발생: {}", e);
    }

    print_separator("3. format! — String 생성");

    let name = "Rust";
    let year = 2026;

    let s1 = format!("{} is awesome in {}!", name, year);
    let s2 = format!("{:>20}", "우측 정렬");
    let s3 = format!("{:=^30}", " 제목 ");     // = 으로 가운데 정렬

    println!("{}", s1);
    println!("[{}]", s2);
    println!("{}", s3);

    // 복잡한 포맷
    let table = format!(
        "{:<15} {:>10} {:>10}\n{:<15} {:>10} {:>10}\n{:<15} {:>10} {:>10}",
        "이름", "점수", "등수",
        "───────────────", "──────────", "──────────",
        "김철수", 95, 2
    );
    println!("{}", table);

    print_separator("4. write! / writeln! — 임의 스트림에 출력");

    // write!는 fmt::Write 또는 io::Write를 구현한 것 어디든 사용
    let mut buffer = String::new();
    use std::fmt::Write as FmtWrite;

    write!(buffer, "버퍼에 쓰기: {}", 42).unwrap();
    writeln!(buffer, " — 개행 포함").unwrap();
    writeln!(buffer, "두 번째 줄").unwrap();

    println!("버퍼 내용:\n{}", buffer);

    // Vec<u8>에 쓰기
    let mut bytes: Vec<u8> = Vec::new();
    write!(bytes, "바이트 버퍼: {}", "hello").unwrap();
    println!("바이트 버퍼: {:?}", String::from_utf8(bytes).unwrap());

    print_separator("5. dbg! — 디버그 전용 매크로");

    // dbg!는 값을 출력하면서 동시에 그 값을 반환 (stderr 출력)
    let x = 5;
    let y = dbg!(x * 2) + 1;  // stderr: [examples/02_print_variants.rs:XX] x * 2 = 10
    println!("y = {}", y);    // y = 11

    let v = vec![1, 2, 3];
    let sum: i32 = dbg!(v).iter().sum();  // v를 출력하면서 이동
    println!("합계: {}", sum);

    print_separator("6. 조건부 출력 (디버그 빌드에서만)");

    // debug_assert! — 릴리스 빌드에서는 제거됨
    debug_assert!(2 + 2 == 4, "수학이 고장났습니다");
    println!("debug_assert 통과");

    // cfg! 매크로로 빌드 타입 확인
    if cfg!(debug_assertions) {
        println!("현재 Debug 빌드입니다");
    } else {
        println!("현재 Release 빌드입니다 (--release 플래그 사용)");
    }
}
