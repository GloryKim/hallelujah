// Mini-Project: 숫자 맞추기 게임
// 실행: cargo run
// 1~100 사이의 랜덤 숫자를 생성하고, 플레이어가 맞출 때까지 힌트를 줌

use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

struct GameState {
    secret: u32,
    attempts: u32,
    max_attempts: u32,
    low: u32,
    high: u32,
}

impl GameState {
    fn new(max_attempts: u32) -> Self {
        let secret = rand::thread_rng().gen_range(1..=100);
        GameState {
            secret,
            attempts: 0,
            max_attempts,
            low: 1,
            high: 100,
        }
    }

    fn guess(&mut self, n: u32) -> GuessResult {
        self.attempts += 1;
        match n.cmp(&self.secret) {
            Ordering::Less => {
                self.low = n + 1;
                GuessResult::TooLow
            }
            Ordering::Greater => {
                self.high = n - 1;
                GuessResult::TooHigh
            }
            Ordering::Equal => GuessResult::Correct,
        }
    }

    fn remaining(&self) -> u32 {
        self.max_attempts.saturating_sub(self.attempts)
    }

    fn hint_range(&self) -> String {
        format!("[{}~{}]", self.low, self.high)
    }
}

enum GuessResult {
    TooLow,
    TooHigh,
    Correct,
}

fn rating(attempts: u32) -> &'static str {
    match attempts {
        1 => "믿을 수 없어요! 천재세요? 🏆",
        2..=3 => "놀라운 실력! ⭐⭐⭐",
        4..=5 => "훌륭합니다! ⭐⭐",
        6..=7 => "잘 하셨어요! ⭐",
        _ => "다음엔 더 잘 할 수 있어요! 💪",
    }
}

fn main() {
    println!("╔════════════════════════════════════╗");
    println!("║     🎮 숫자 맞추기 게임 🎮          ║");
    println!("║  1에서 100 사이 숫자를 맞춰보세요!  ║");
    println!("╚════════════════════════════════════╝\n");

    let max_attempts = 10u32;
    let stdin = io::stdin();
    let mut game = GameState::new(max_attempts);

    println!("{}번 안에 맞춰보세요!", max_attempts);
    println!("(힌트: 이진 탐색으로 최대 7번이면 됩니다)\n");

    loop {
        print!(
            "추측 #{} {} 숫자 입력: ",
            game.attempts + 1,
            game.hint_range()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => {
                println!("\n입력이 없어서 게임을 종료합니다.");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("입력 오류: {}", e);
                break;
            }
        }

        let trimmed = input.trim();

        // 명령어 처리
        match trimmed {
            "quit" | "q" | "종료" => {
                println!("게임 종료! 정답은 {}이었어요.", game.secret);
                break;
            }
            "hint" => {
                println!("힌트: 정답은 {}의 배수입니까? {}",
                    3,
                    if game.secret % 3 == 0 { "예" } else { "아니요" }
                );
                continue;
            }
            _ => {}
        }

        // 숫자 파싱
        let guess: u32 = match trimmed.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("  ⚠️  숫자를 입력하세요! (또는 'quit'으로 종료)\n");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("  ⚠️  1에서 100 사이 숫자를 입력하세요!\n");
            continue;
        }

        match game.guess(guess) {
            GuessResult::TooLow => {
                println!("  📉 너무 작아요! {} 이상을 시도해보세요.", game.low);
            }
            GuessResult::TooHigh => {
                println!("  📈 너무 커요! {} 이하를 시도해보세요.", game.high);
            }
            GuessResult::Correct => {
                println!(
                    "\n  🎉 정답! {} = {}번 만에 맞췄어요!",
                    game.secret, game.attempts
                );
                println!("  {}", rating(game.attempts));
                break;
            }
        }

        let remaining = game.remaining();
        if remaining == 0 {
            println!("\n  😢 기회를 모두 사용했어요! 정답은 {}이었어요.", game.secret);
            break;
        }

        println!("  남은 기회: {}\n", remaining);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_guess() {
        let mut game = GameState::new(10);
        let secret = game.secret;
        match game.guess(secret) {
            GuessResult::Correct => {}
            _ => panic!("정답 처리 실패"),
        }
        assert_eq!(game.attempts, 1);
    }

    #[test]
    fn test_attempts_count() {
        let mut game = GameState::new(10);
        assert_eq!(game.attempts, 0);
        game.guess(50);
        assert_eq!(game.attempts, 1);
        game.guess(50);
        assert_eq!(game.attempts, 2);
    }

    #[test]
    fn test_range_narrows() {
        let mut game = GameState::new(10);
        game.secret = 75;  // 테스트를 위해 직접 설정

        game.guess(50);  // TooLow
        assert_eq!(game.low, 51);

        game.guess(90);  // TooHigh
        assert_eq!(game.high, 89);
    }

    #[test]
    fn test_rating() {
        assert_eq!(rating(1), "믿을 수 없어요! 천재세요? 🏆");
        assert!(rating(5).contains("훌륭합니다"));
        assert!(rating(15).contains("다음엔"));
    }

    #[test]
    fn test_remaining() {
        let mut game = GameState::new(5);
        assert_eq!(game.remaining(), 5);
        game.guess(50);
        assert_eq!(game.remaining(), 4);
    }
}
