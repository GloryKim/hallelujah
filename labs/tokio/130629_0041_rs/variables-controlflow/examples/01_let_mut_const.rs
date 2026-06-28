// 예제 01: let, let mut, const, static
// 실행: cargo run --example 01_let_mut_const

static PROGRAM_NAME: &str = "Rust 변수 예제";
static mut GLOBAL_COUNTER: u32 = 0;

const MAX_HEALTH: u32 = 100;
const MIN_HEALTH: i32 = 0;
const GRAVITY: f64 = 9.81;
const PI: f64 = std::f64::consts::PI;

const fn buffer_size(kb: usize) -> usize {
    kb * 1024
}
const BUFFER: usize = buffer_size(8);  // 8192

fn main() {
    println!("=== {} ===\n", PROGRAM_NAME);

    // ── 불변 변수 ──────────────────────────────────────────────
    println!("── 불변 변수 (let) ──");

    let x = 5;
    let y = 10;
    let sum = x + y;
    println!("x = {}, y = {}, x + y = {}", x, y, sum);

    // x = 6;  // 오류: cannot assign twice to immutable variable

    // ── 가변 변수 ──────────────────────────────────────────────
    println!("\n── 가변 변수 (let mut) ──");

    let mut count = 0u32;
    println!("초기값: {}", count);
    count += 1;
    println!("+1 후: {}", count);
    count *= 10;
    println!("*10 후: {}", count);

    let mut health = MAX_HEALTH;
    println!("\n체력: {}/{}", health, MAX_HEALTH);
    health = health.saturating_sub(30);  // 데미지
    println!("30 데미지: {}/{}", health, MAX_HEALTH);
    health = (health + 20).min(MAX_HEALTH);  // 회복 (최댓값 제한)
    println!("20 회복: {}/{}", health, MAX_HEALTH);

    // ── 상수 ───────────────────────────────────────────────────
    println!("\n── 상수 (const) ──");
    println!("MAX_HEALTH: {}", MAX_HEALTH);
    println!("MIN_HEALTH: {}", MIN_HEALTH);
    println!("GRAVITY: {} m/s²", GRAVITY);
    println!("PI: {:.10}", PI);
    println!("BUFFER: {} bytes ({} KB)", BUFFER, BUFFER / 1024);

    // 상수는 어디서든 사용 가능 (함수 내 정의도 OK)
    const LOCAL_CONST: &str = "함수 내 상수도 가능";
    println!("{}", LOCAL_CONST);

    // ── 정적 변수 ──────────────────────────────────────────────
    println!("\n── 정적 변수 (static) ──");
    println!("프로그램 이름: {}", PROGRAM_NAME);

    // 가변 정적 변수는 unsafe (글로벌 상태는 데이터 레이스 가능)
    // static_mut_refs: 교육용으로 의도된 코드 — 실무에서는 Mutex/AtomicUsize 사용
    #[allow(static_mut_refs)]
    unsafe {
        GLOBAL_COUNTER += 1;
        println!("글로벌 카운터: {}", GLOBAL_COUNTER);
        GLOBAL_COUNTER += 1;
        println!("글로벌 카운터: {}", GLOBAL_COUNTER);
    }

    // ── 실용 패턴: 초기화 후 불변 ─────────────────────────────
    println!("\n── 초기화 후 불변 패턴 ──");

    // mut으로 복잡하게 구성한 뒤, 불변으로 "잠금"
    let data = {
        let mut temp = Vec::new();
        temp.push(1);
        temp.push(2);
        temp.push(3);
        temp  // 블록의 마지막 표현식 = 반환값
    };
    // data는 불변! (이후 수정 불가)
    println!("불변 데이터: {:?}", data);

    // ── 변수 할당과 표현식 ────────────────────────────────────
    println!("\n── 모든 것은 표현식 ──");

    let a = {
        let base = 10;
        base * base + 1  // 세미콜론 없음 = 표현식 = 반환
    };
    println!("블록 표현식 결과: {}", a);  // 101

    let b = if true { 42 } else { 0 };
    println!("if 표현식 결과: {}", b);

    // ── 언더스코어 변수 ────────────────────────────────────────
    println!("\n── 특수 변수명 ──");

    let _unused = "이 변수는 사용되지 않아도 경고 없음";
    let _ = "이 값은 완전히 무시됨";

    // 여러 변수 동시 선언 (구조 분해)
    let (p, q, r) = (1, 2, 3);
    println!("p={}, q={}, r={}", p, q, r);

    let [first, second, ..] = [10, 20, 30, 40, 50][..] else {
        panic!("매칭 실패");
    };
    println!("first={}, second={}", first, second);
}
