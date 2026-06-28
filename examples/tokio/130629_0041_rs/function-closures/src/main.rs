// Mini-Project: 단어 빈도 분석기
// 실행: cargo run
// 하드코딩된 텍스트에서 단어 빈도를 계산하고 상위 5개를 출력

use std::collections::HashMap;

const TEXT: &str = r#"
Rust is a systems programming language focused on three goals:
safety, speed, and concurrency. It accomplishes these goals without
a garbage collector, making it useful for a number of use cases other
languages aren't good at: embedding in other languages, programs with
specific space and time requirements, and writing low-level code, like
device drivers and operating systems.

Rust improves on current languages targeting this space by having a
number of compile-time safety checks with no runtime overhead, while
eliminating all data races. Rust also aims to achieve zero-cost
abstractions even though some of these abstractions feel like those
of high-level languages. Even then, Rust still allows the fine-grained
control that is expected of a low-level language.

The Rust programming language is designed for systems programming.
Rust helps eliminate whole categories of programming errors and memory
bugs that are common in systems languages like C and C++. Rust achieves
memory safety without using a garbage collector or runtime.
"#;

fn normalize(word: &str) -> String {
    word.to_lowercase()
        .trim_matches(|c: char| !c.is_alphabetic())
        .to_string()
}

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut freq: HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let normalized = normalize(word);
        if !normalized.is_empty() && normalized.len() > 1 {
            *freq.entry(normalized).or_insert(0) += 1;
        }
    }

    freq
}

fn top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(&String, &usize)> {
    let mut entries: Vec<(&String, &usize)> = freq.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    entries.into_iter().take(n).collect()
}

fn text_stats(text: &str) -> (usize, usize, usize, usize) {
    let chars = text.chars().filter(|c| !c.is_whitespace()).count();
    let words = text.split_whitespace().count();
    let lines = text.lines().filter(|l| !l.trim().is_empty()).count();
    let sentences = text.chars().filter(|&c| c == '.' || c == '!' || c == '?').count();
    (chars, words, lines, sentences)
}

fn histogram(word: &str, count: usize, max_count: usize, bar_width: usize) -> String {
    let filled = (count * bar_width) / max_count;
    let bar = "█".repeat(filled) + &"░".repeat(bar_width - filled);
    format!("{:<20} {:>4} | {}", word, count, bar)
}

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║       📊 단어 빈도 분석기                 ║");
    println!("╚══════════════════════════════════════════╝\n");

    // 기본 통계
    let (chars, words, lines, sentences) = text_stats(TEXT);
    println!("── 텍스트 통계 ──");
    println!("  문자 수:   {}", chars);
    println!("  단어 수:   {}", words);
    println!("  문장 수:   {}", sentences);
    println!("  줄 수:     {}", lines);

    // 단어 빈도 계산
    let freq = count_words(TEXT);
    println!("\n  고유 단어: {}개\n", freq.len());

    // 상위 5개
    let top5 = top_n(&freq, 5);
    let max_count = top5.first().map(|(_, &c)| c).unwrap_or(1);

    println!("── 상위 5개 단어 (빈도순) ──\n");
    println!("{:<20} {:>4}   빈도 그래프", "단어", "횟수");
    println!("{}", "─".repeat(55));

    for (rank, (word, &count)) in top5.iter().enumerate() {
        let hist = histogram(word, count, max_count, 20);
        println!("#{}: {}", rank + 1, hist);
    }

    // 상위 20개 목록
    println!("\n── 상위 20개 단어 ──\n");
    let top20 = top_n(&freq, 20);
    for (i, (word, &count)) in top20.iter().enumerate() {
        print!("{:>2}. {:<15} {:>3}번   ", i + 1, word, count);
        if (i + 1) % 2 == 0 { println!(); }
    }
    println!();

    // 빈도별 분포
    println!("\n── 빈도 분포 ──");
    let mut dist: HashMap<usize, usize> = HashMap::new();
    for &count in freq.values() {
        *dist.entry(count).or_insert(0) += 1;
    }
    let mut dist_vec: Vec<_> = dist.iter().collect();
    dist_vec.sort_by_key(|(&k, _)| k);

    for (&freq_count, &word_count) in dist_vec.iter().take(10) {
        let bar = "▪".repeat(word_count.min(20));
        println!("  {}번 등장: {:>3}개 단어  {}", freq_count, word_count, bar);
    }

    // 특정 단어 검색
    println!("\n── 특정 단어 검색 ──");
    let search_words = ["rust", "safety", "memory", "language", "the"];
    for word in &search_words {
        let count = freq.get(*word).copied().unwrap_or(0);
        println!("  \"{}\": {}번", word, count);
    }

    // 이터레이터 체인으로 분석
    println!("\n── 이터레이터 체인 분석 ──");

    // 5번 이상 등장한 단어
    let frequent: Vec<_> = freq.iter()
        .filter(|(_, &v)| v >= 3)
        .map(|(k, v)| (k.as_str(), v))
        .collect();
    println!("3번 이상 등장한 단어: {}개", frequent.len());

    // 긴 단어 (8자 이상) 중 2번 이상
    let long_words: Vec<_> = freq.iter()
        .filter(|(k, &v)| k.len() >= 8 && v >= 2)
        .map(|(k, v)| format!("{} ({}번, {}자)", k, v, k.len()))
        .collect();
    println!("8자 이상 + 2번 이상:");
    for w in &long_words {
        println!("  {}", w);
    }

    // 알파벳 첫 글자별 단어 수
    println!("\n알파벳 시작 분포:");
    let mut by_initial: HashMap<char, usize> = HashMap::new();
    for word in freq.keys() {
        if let Some(c) = word.chars().next() {
            *by_initial.entry(c).or_insert(0) += 1;
        }
    }
    let mut alpha_vec: Vec<_> = by_initial.iter().collect();
    alpha_vec.sort_by_key(|(&c, _)| c);
    for (&c, &count) in &alpha_vec {
        print!("{}:{} ", c, count);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("Hello,"), "hello");
        assert_eq!(normalize("world."), "world");
        assert_eq!(normalize("Rust!"), "rust");
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn test_count_words_basic() {
        let freq = count_words("hello world hello rust");
        assert_eq!(freq.get("hello"), Some(&2));
        assert_eq!(freq.get("world"), Some(&1));
        assert_eq!(freq.get("rust"), Some(&1));
    }

    #[test]
    fn test_top_n() {
        let freq = count_words("a a a b b c");
        let top = top_n(&freq, 2);
        assert_eq!(top[0].0, "a");
        assert_eq!(*top[0].1, 3);
    }

    #[test]
    fn test_text_stats() {
        let (_, words, _, _) = text_stats("hello world foo");
        assert_eq!(words, 3);
    }
}
