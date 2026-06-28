// Mini-Project: 타임스탬프 인사 CLI
// 실행: cargo run
// 실행: cargo run -- "철수"
// 실행: cargo run -- "영희" "반갑습니다"

use chrono::Local;
use std::env;

fn get_time_greeting(hour: u32) -> &'static str {
    match hour {
        5..=11 => "좋은 아침이에요",
        12..=17 => "좋은 오후에요",
        18..=21 => "좋은 저녁이에요",
        _ => "안녕하세요",
    }
}

fn build_greeting(name: &str, custom_msg: Option<&str>) -> String {
    let now = Local::now();
    let hour = now.format("%H").to_string().parse::<u32>().unwrap_or(12);
    let time_str = now.format("%Y년 %m월 %d일 (%A) %H:%M:%S").to_string();
    let greeting = get_time_greeting(hour);

    match custom_msg {
        Some(msg) => format!(
            "╔══════════════════════════════════════╗\n\
             ║  {}, {}!                \n\
             ║  {}              \n\
             ║  현재 시각: {}   \n\
             ╚══════════════════════════════════════╝",
            msg, name, greeting, time_str
        ),
        None => format!(
            "╔══════════════════════════════════════╗\n\
             ║  안녕하세요, {}님!             \n\
             ║  {}              \n\
             ║  현재 시각: {}   \n\
             ╚══════════════════════════════════════╝",
            name, greeting, time_str
        ),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = if args.len() > 1 {
        args[1].clone()
    } else {
        env::var("GREETING_NAME").unwrap_or_else(|_| String::from("세계"))
    };

    let custom_msg = if args.len() > 2 {
        Some(args[2].as_str())
    } else {
        None
    };

    println!("{}", build_greeting(&name, custom_msg));

    println!("\nRust 버전 정보:");
    println!("  - 에디션: 2021");
    println!("  - 빌드: {}", if cfg!(debug_assertions) { "Debug" } else { "Release" });

    #[cfg(target_os = "linux")]
    println!("  - OS: Linux");
    #[cfg(target_os = "macos")]
    println!("  - OS: macOS");
    #[cfg(target_os = "windows")]
    println!("  - OS: Windows");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morning_greeting() {
        assert_eq!(get_time_greeting(9), "좋은 아침이에요");
    }

    #[test]
    fn test_afternoon_greeting() {
        assert_eq!(get_time_greeting(14), "좋은 오후에요");
    }

    #[test]
    fn test_evening_greeting() {
        assert_eq!(get_time_greeting(19), "좋은 저녁이에요");
    }

    #[test]
    fn test_night_greeting() {
        assert_eq!(get_time_greeting(23), "안녕하세요");
    }

    #[test]
    fn test_greeting_contains_name() {
        let result = build_greeting("철수", None);
        assert!(result.contains("철수"));
    }

    #[test]
    fn test_greeting_with_custom_message() {
        let result = build_greeting("영희", Some("환영합니다"));
        assert!(result.contains("영희"));
        assert!(result.contains("환영합니다"));
    }
}
