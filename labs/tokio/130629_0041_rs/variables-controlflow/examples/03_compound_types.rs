// 예제 03: 복합 타입 — 튜플, 배열, 슬라이스
// 실행: cargo run --example 03_compound_types

fn largest(slice: &[i32]) -> i32 {
    let mut max = slice[0];
    for &val in &slice[1..] {
        if val > max {
            max = val;
        }
    }
    max
}

fn sum_slice(s: &[f64]) -> f64 {
    s.iter().sum()
}

fn reverse_array<T: Copy, const N: usize>(arr: [T; N]) -> [T; N] {
    let mut result = arr;
    result.reverse();
    result
}

fn main() {
    println!("=== 복합 타입: 튜플, 배열, 슬라이스 ===\n");

    // ════════════════════════════════════════════════
    println!("── 1. 튜플 (Tuple) ──");
    // ════════════════════════════════════════════════

    // 다른 타입을 묶을 수 있음
    let point: (f64, f64) = (3.0, 4.0);
    let person: (&str, u32, bool) = ("Alice", 30, true);
    let rgb: (u8, u8, u8) = (255, 128, 0);
    let unit: () = ();  // 유닛 타입

    println!("point:  {:?}", point);
    println!("person: {:?}", person);
    println!("rgb:    ({}, {}, {})", rgb.0, rgb.1, rgb.2);
    println!("unit:   {:?}", unit);

    // 인덱스 접근 (.0, .1, .2, ...)
    println!("\n인덱스 접근:");
    println!("이름: {}", person.0);
    println!("나이: {}", person.1);
    println!("취업: {}", person.2);

    // 구조 분해 (destructuring)
    let (name, age, employed) = person;
    println!("\n구조 분해: {} {} {}", name, age, employed);

    // 일부만 구조 분해
    let (x, y) = point;
    let distance = (x * x + y * y).sqrt();
    println!("원점에서 ({}, {})까지의 거리: {:.2}", x, y, distance);

    // 함수에서 여러 값 반환
    fn min_max(data: &[i32]) -> (i32, i32) {
        let min = *data.iter().min().unwrap();
        let max = *data.iter().max().unwrap();
        (min, max)
    }

    let data = [5, 2, 8, 1, 9, 3, 7];
    let (min, max) = min_max(&data);
    println!("min={}, max={}", min, max);

    // 중첩 튜플
    let nested = ((1, 2), (3, 4), (5, 6));
    println!("중첩 튜플 .0.1 = {}", nested.0 .1);

    // 튜플 구조체 (named tuple)
    struct Color(u8, u8, u8);
    struct Point2D(f32, f32);

    let red = Color(255, 0, 0);
    let origin = Point2D(0.0, 0.0);
    println!("Red: rgb({}, {}, {})", red.0, red.1, red.2);
    println!("Origin: ({}, {})", origin.0, origin.1);

    // ════════════════════════════════════════════════
    println!("\n── 2. 배열 (Array) — 고정 크기, 스택 할당 ──");
    // ════════════════════════════════════════════════

    // 선언 방법
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [0; 10];          // 0으로 채운 10개
    let arr3 = [true; 4];        // [true, true, true, true]
    let arr4: [f64; 3] = [1.1, 2.2, 3.3];

    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);
    println!("arr3: {:?}", arr3);
    println!("arr4: {:?}", arr4);

    // 길이와 접근
    println!("\n배열 정보:");
    println!("arr1.len() = {}", arr1.len());
    println!("arr1[0]    = {}", arr1[0]);
    println!("arr1[4]    = {}", arr1[4]);

    // 안전한 인덱싱
    let idx = 10usize;
    match arr1.get(idx) {
        Some(val) => println!("arr1[{}] = {}", idx, val),
        None => println!("arr1[{}]: 인덱스 범위 초과!", idx),
    }

    // 반복
    print!("for 반복: ");
    for elem in arr1.iter() {
        print!("{} ", elem);
    }
    println!();

    // 인덱스 + 값
    println!("enumerate:");
    for (i, val) in arr1.iter().enumerate() {
        print!("[{}]={} ", i, val);
    }
    println!();

    // 가변 배열
    let mut scores = [85, 92, 78, 95, 88];
    scores[2] = 82;
    println!("\n점수: {:?}", scores);

    // 배열 메서드
    let nums = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    println!("contains(5): {}", nums.contains(&5));
    println!("iter().sum(): {}", nums.iter().sum::<i32>());
    println!("iter().max(): {:?}", nums.iter().max());
    println!("iter().min(): {:?}", nums.iter().min());

    // 정렬
    let mut sortable = [5, 2, 8, 1, 9, 3];
    sortable.sort();
    println!("정렬 후: {:?}", sortable);

    let mut desc = [5, 2, 8, 1, 9, 3];
    desc.sort_by(|a, b| b.cmp(a));
    println!("내림차순: {:?}", desc);

    // 제네릭 배열 reverse
    let reversed = reverse_array([1, 2, 3, 4, 5]);
    println!("역순: {:?}", reversed);

    // 2D 배열
    let matrix: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    println!("\n2D 배열 (3×3 행렬):");
    for row in &matrix {
        for val in row {
            print!("{:3}", val);
        }
        println!();
    }

    // ════════════════════════════════════════════════
    println!("\n── 3. 슬라이스 (&[T]) ──");
    // ════════════════════════════════════════════════

    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // 슬라이스 범위
    let s1 = &arr[2..5];      // [30, 40, 50]
    let s2 = &arr[..3];       // [10, 20, 30]
    let s3 = &arr[7..];       // [80, 90, 100]
    let s_all = &arr[..];     // 전체

    println!("arr[2..5]:  {:?}", s1);
    println!("arr[..3]:   {:?}", s2);
    println!("arr[7..]:   {:?}", s3);
    println!("arr[..]:    {:?}", s_all);

    // 슬라이스는 포인터 + 길이
    println!("\n슬라이스 메타데이터:");
    println!("s1.len() = {}", s1.len());
    println!("s1.is_empty() = {}", s1.is_empty());
    println!("s1.first() = {:?}", s1.first());
    println!("s1.last()  = {:?}", s1.last());

    // 슬라이스를 인자로 받는 함수
    println!("\n슬라이스 함수:");
    println!("largest(&arr) = {}", largest(&arr));
    println!("largest(s1)   = {}", largest(s1));

    let floats = [1.1, 2.2, 3.3, 4.4];
    println!("sum_slice = {}", sum_slice(&floats));

    // 가변 슬라이스
    let mut data = [1, 2, 3, 4, 5];
    let mutable_slice = &mut data[1..4];
    for v in mutable_slice.iter_mut() {
        *v *= 10;
    }
    println!("\n가변 슬라이스 수정 후: {:?}", data);

    // 슬라이스 분할
    let (left, right) = arr.split_at(5);
    println!("split_at(5): {:?} | {:?}", left, right);

    // chunks — N개씩 묶어서 처리
    println!("\nchunks(3):");
    for chunk in arr.chunks(3) {
        println!("  {:?}", chunk);
    }

    // windows — N개 슬라이딩 윈도우
    println!("\nwindows(3) (처음 5개):");
    for window in arr.windows(3).take(5) {
        println!("  {:?}", window);
    }

    // 문자열 슬라이스
    println!("\n── 4. 문자열 슬라이스 (&str) ──");

    let sentence = String::from("The quick brown fox");
    let first_word = &sentence[..3];   // "The"
    let rest = &sentence[4..];         // "quick brown fox"

    println!("전체: \"{}\"", sentence);
    println!("처음 단어: \"{}\"", first_word);
    println!("나머지: \"{}\"", rest);

    // 문자열 슬라이스 주의: UTF-8 바이트 경계
    let korean = "안녕하세요";
    // UTF-8에서 한글 1글자 = 3바이트
    // &korean[0..1]  // 패닉! 1바이트는 불완전한 글자
    let first_char = &korean[0..3];  // "안" (3바이트)
    println!("한글 첫 글자: {}", first_char);

    // 안전한 방법: chars() 사용
    let chars: Vec<char> = korean.chars().collect();
    println!("한글 문자 배열: {:?}", chars);
}
