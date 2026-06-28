// 예제 05: chrono 크레이트로 날짜/시간 다루기
// 실행: cargo run --example 05_chrono_timestamp

use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, TimeZone,
    Timelike, Utc, Weekday,
};

fn print_section(title: &str) {
    println!("\n{}", "═".repeat(50));
    println!("  {}", title);
    println!("{}", "═".repeat(50));
}

fn main() {
    print_section("1. 현재 시간");

    let now_local: DateTime<Local> = Local::now();
    let now_utc: DateTime<Utc> = Utc::now();

    println!("로컬 시간:  {}", now_local);
    println!("UTC 시간:   {}", now_utc);
    println!("RFC 2822:   {}", now_local.to_rfc2822());
    println!("RFC 3339:   {}", now_local.to_rfc3339());

    print_section("2. 날짜/시간 컴포넌트 분해");

    println!("년도:   {}", now_local.year());
    println!("월:     {}", now_local.month());
    println!("일:     {}", now_local.day());
    println!("시:     {}", now_local.hour());
    println!("분:     {}", now_local.minute());
    println!("초:     {}", now_local.second());
    println!("나노초: {}", now_local.nanosecond());
    println!("요일:   {:?}", now_local.weekday());
    println!("올해의 몇 번째 날: {}", now_local.ordinal());
    println!("올해의 몇 주차: {}", now_local.iso_week().week());

    // 요일 한국어
    let day_kor = match now_local.weekday() {
        Weekday::Mon => "월요일",
        Weekday::Tue => "화요일",
        Weekday::Wed => "수요일",
        Weekday::Thu => "목요일",
        Weekday::Fri => "금요일",
        Weekday::Sat => "토요일",
        Weekday::Sun => "일요일",
    };
    println!("요일(한국어): {}", day_kor);

    print_section("3. 포맷 문자열");

    // strftime 형식
    let formats = [
        ("%Y-%m-%d",              "YYYY-MM-DD"),
        ("%Y/%m/%d %H:%M:%S",     "YYYY/MM/DD HH:MM:SS"),
        ("%d %B %Y",              "DD Month YYYY (영문)"),
        ("%a, %d %b %Y",          "요일약어, DD Mon YYYY"),
        ("%I:%M %p",              "12시간제"),
        ("%H:%M:%S",              "24시간제"),
        ("%Y년 %m월 %d일",        "한국식 날짜"),
        ("%Y년 %m월 %d일 %H시 %M분 %S초", "한국식 날짜+시간"),
        ("%s",                    "Unix 타임스탬프 (초)"),
        ("%3f",                   "밀리초"),
        ("%6f",                   "마이크로초"),
    ];

    for (fmt, desc) in &formats {
        println!("{:<35} → {}", format!("\"{}\"", fmt), now_local.format(fmt));
        let _ = desc; // 실제로는 desc 출력
    }

    // desc 포함 버전
    println!("\n형식 설명:");
    for (fmt, desc) in &formats {
        println!("  {:25} {:30} → {}", fmt, desc, now_local.format(fmt));
    }

    print_section("4. 날짜 연산");

    let today = now_local;

    // 더하기/빼기
    let tomorrow = today + Duration::days(1);
    let yesterday = today - Duration::days(1);
    let next_week = today + Duration::weeks(1);
    let next_month = today + Duration::days(30);
    let next_hour = today + Duration::hours(1);

    println!("오늘:    {}", today.format("%Y-%m-%d %H:%M"));
    println!("내일:    {}", tomorrow.format("%Y-%m-%d"));
    println!("어제:    {}", yesterday.format("%Y-%m-%d"));
    println!("다음 주: {}", next_week.format("%Y-%m-%d"));
    println!("30일 후: {}", next_month.format("%Y-%m-%d"));
    println!("1시간 후: {}", next_hour.format("%H:%M"));

    // 두 날짜 사이의 차이
    let target = Local.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
    let diff = target.signed_duration_since(today);

    println!("\n2027-01-01까지:");
    println!("  {} 일", diff.num_days());
    println!("  {} 시간", diff.num_hours());
    println!("  {} 분", diff.num_minutes());

    print_section("5. NaiveDate / NaiveDateTime (타임존 없음)");

    // NaiveDate — 타임존 정보 없는 날짜 (데이터베이스 저장 등에 유용)
    let birth = NaiveDate::from_ymd_opt(1990, 6, 15).unwrap();
    let today_naive = Local::now().date_naive();
    let age = today_naive.signed_duration_since(birth).num_days() / 365;

    println!("생일: {}", birth);
    println!("나이: {} 살", age);

    // NaiveDateTime — 타임존 없는 날짜+시간
    let dt = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2026, 5, 22).unwrap(),
        chrono::NaiveTime::from_hms_opt(13, 59, 0).unwrap(),
    );
    println!("NaiveDateTime: {}", dt);

    print_section("6. 문자열 파싱");

    // 문자열 → DateTime
    let parsed1 = "2026-05-22 14:30:00"
        .parse::<NaiveDateTime>();
    println!("파싱 결과: {:?}", parsed1);

    let parsed2 = DateTime::parse_from_rfc3339("2026-05-22T14:30:00+09:00");
    println!("RFC3339 파싱: {:?}", parsed2.map(|d| d.to_string()));

    let parsed3 = DateTime::parse_from_str(
        "2026-05-22 14:30:00 +0900",
        "%Y-%m-%d %H:%M:%S %z",
    );
    println!("커스텀 파싱: {:?}", parsed3.map(|d| d.to_string()));

    print_section("7. Unix 타임스탬프");

    let ts = now_local.timestamp();           // 초
    let ts_ms = now_local.timestamp_millis(); // 밀리초
    let ts_us = now_local.timestamp_micros(); // 마이크로초

    println!("Unix timestamp (초): {}", ts);
    println!("Unix timestamp (ms): {}", ts_ms);
    println!("Unix timestamp (μs): {}", ts_us);

    // 타임스탬프 → DateTime
    let from_ts = DateTime::from_timestamp(ts, 0).unwrap();
    println!("타임스탬프 역변환: {}", from_ts.with_timezone(&Local));
}
