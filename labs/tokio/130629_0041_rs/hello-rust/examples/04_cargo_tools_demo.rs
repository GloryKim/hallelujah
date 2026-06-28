// 예제 04: cargo 도구 사용법 데모 + 환경 정보
// 실행: cargo run --example 04_cargo_tools_demo

fn main() {
    println!("=== Cargo 도구 사용법 안내 ===\n");

    // 컴파일 타임 상수
    println!("── 컴파일 타임 환경 정보 ──");
    println!("패키지 이름:    {}", env!("CARGO_PKG_NAME"));
    println!("패키지 버전:    {}", env!("CARGO_PKG_VERSION"));
    println!("Rust 에디션:    2021");
    println!("컴파일러 채널:  stable");

    println!("\n── 조건부 컴파일 ──");

    #[cfg(debug_assertions)]
    println!("빌드 모드: Debug (cargo build)");

    #[cfg(not(debug_assertions))]
    println!("빌드 모드: Release (cargo build --release)");

    #[cfg(target_os = "linux")]
    println!("운영체제: Linux");

    #[cfg(target_os = "macos")]
    println!("운영체제: macOS");

    #[cfg(target_os = "windows")]
    println!("운영체제: Windows");

    #[cfg(target_arch = "x86_64")]
    println!("아키텍처: x86_64 (64비트)");

    #[cfg(target_arch = "aarch64")]
    println!("아키텍처: ARM64 (Apple Silicon 또는 ARM)");

    println!("\n── 포인터 크기 ──");
    println!("usize 크기: {} 바이트", std::mem::size_of::<usize>());
    println!("포인터 크기: {} 바이트", std::mem::size_of::<*const u8>());

    println!("\n── cargo 명령어 치트시트 ──");
    let commands = [
        ("cargo new <이름>",          "새 바이너리 프로젝트 생성"),
        ("cargo new <이름> --lib",    "새 라이브러리 프로젝트 생성"),
        ("cargo build",               "디버그 빌드"),
        ("cargo build --release",     "릴리스 빌드 (최적화)"),
        ("cargo run",                 "빌드 후 실행"),
        ("cargo run --example <이름>","예제 실행"),
        ("cargo check",               "빌드 없이 오류 검사 (빠름)"),
        ("cargo test",                "모든 테스트 실행"),
        ("cargo fmt",                 "코드 포맷팅"),
        ("cargo clippy",              "린트 분석"),
        ("cargo add <크레이트>",      "의존성 추가"),
        ("cargo doc --open",          "문서 생성 + 브라우저 열기"),
        ("cargo clean",               "빌드 산출물 제거"),
        ("cargo update",              "의존성 업데이트"),
        ("cargo tree",                "의존성 트리 출력"),
    ];

    println!("{:<35} {}", "명령어", "설명");
    println!("{}", "─".repeat(70));
    for (cmd, desc) in &commands {
        println!("{:<35} {}", cmd, desc);
    }

    println!("\n── rustup 명령어 치트시트 ──");
    let rustup_commands = [
        ("rustup update",                "모든 툴체인 업데이트"),
        ("rustup default stable",        "stable 채널을 기본으로"),
        ("rustup install nightly",       "nightly 툴체인 설치"),
        ("rustup toolchain list",        "설치된 툴체인 목록"),
        ("rustup target add <타겟>",     "크로스 컴파일 타겟 추가"),
        ("rustup component add clippy",  "clippy 컴포넌트 추가"),
        ("rustup component add rustfmt", "rustfmt 컴포넌트 추가"),
        ("rustup show",                  "현재 툴체인 정보"),
    ];

    println!("{:<35} {}", "명령어", "설명");
    println!("{}", "─".repeat(70));
    for (cmd, desc) in &rustup_commands {
        println!("{:<35} {}", cmd, desc);
    }

    println!("\n── clippy가 잡아주는 패턴 예시 ──");

    // clippy::len_zero — is_empty() 권장
    let v: Vec<i32> = vec![];
    // 나쁨: v.len() == 0
    // 좋음:
    if v.is_empty() {
        println!("벡터가 비었습니다 (is_empty() 사용)");
    }

    // clippy::needless_return — 불필요한 return
    fn add_good(a: i32, b: i32) -> i32 {
        a + b  // return 키워드 불필요 (마지막 표현식이 반환값)
    }
    println!("1 + 2 = {}", add_good(1, 2));

    // clippy::redundant_clone — 불필요한 clone
    let s = String::from("hello");
    let _owned: String = s;  // s.clone() 없이도 이동 가능

    // clippy::single_match — if let 권장
    let opt: Option<i32> = Some(42);
    if let Some(val) = opt {
        println!("값: {} (if let 사용)", val);
    }
}
