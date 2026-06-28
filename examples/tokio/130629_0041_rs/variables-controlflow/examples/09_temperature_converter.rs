// 예제 09: 온도 변환기 (함수 + match + 포맷팅)
// 실행: cargo run --example 09_temperature_converter

#[derive(Debug, Clone, Copy)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
    Rankine(f64),
}

impl Temperature {
    fn to_celsius(self) -> f64 {
        match self {
            Temperature::Celsius(c) => c,
            Temperature::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
            Temperature::Kelvin(k) => k - 273.15,
            Temperature::Rankine(r) => (r - 491.67) * 5.0 / 9.0,
        }
    }

    fn to_fahrenheit(self) -> f64 {
        match self {
            Temperature::Celsius(c) => c * 9.0 / 5.0 + 32.0,
            Temperature::Fahrenheit(f) => f,
            Temperature::Kelvin(k) => (k - 273.15) * 9.0 / 5.0 + 32.0,
            Temperature::Rankine(r) => r - 459.67,
        }
    }

    fn to_kelvin(self) -> f64 {
        match self {
            Temperature::Celsius(c) => c + 273.15,
            Temperature::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0 + 273.15,
            Temperature::Kelvin(k) => k,
            Temperature::Rankine(r) => r * 5.0 / 9.0,
        }
    }

    fn to_rankine(self) -> f64 {
        match self {
            Temperature::Celsius(c) => (c + 273.15) * 9.0 / 5.0,
            Temperature::Fahrenheit(f) => f + 459.67,
            Temperature::Kelvin(k) => k * 9.0 / 5.0,
            Temperature::Rankine(r) => r,
        }
    }

    fn symbol(&self) -> &str {
        match self {
            Temperature::Celsius(_) => "°C",
            Temperature::Fahrenheit(_) => "°F",
            Temperature::Kelvin(_) => "K",
            Temperature::Rankine(_) => "°R",
        }
    }

    fn value(&self) -> f64 {
        match self {
            Temperature::Celsius(v) | Temperature::Fahrenheit(v)
            | Temperature::Kelvin(v) | Temperature::Rankine(v) => *v,
        }
    }

    fn describe(&self) -> &str {
        let celsius = self.to_celsius();
        match celsius as i64 {
            c if c < -273 => "절대 영도 이하 (불가능)",
            -273 => "절대 영도",
            c if c < -89 => "기록된 최저 기온 이하",
            c if c < 0 => "영하",
            0 => "물의 어는점",
            c if c < 20 => "서늘함",
            c if c < 30 => "쾌적함",
            c if c < 40 => "더움",
            c if c < 100 => "매우 더움",
            100 => "물의 끓는점",
            _ => "매우 고온",
        }
    }
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}{}", self.value(), self.symbol())
    }
}

fn print_conversion_table(temp: Temperature) {
    println!("입력: {}", temp);
    println!("  → 섭씨:      {:.4}°C", temp.to_celsius());
    println!("  → 화씨:      {:.4}°F", temp.to_fahrenheit());
    println!("  → 켈빈:      {:.4} K", temp.to_kelvin());
    println!("  → 랭킨:      {:.4} °R", temp.to_rankine());
    println!("  설명: {}", temp.describe());
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("=== 온도 변환기 ===\n");

    // ── 기본 변환 ──────────────────────────────────────────────
    println!("── 1. 주요 기준점 변환 ──\n");

    let reference_points = [
        Temperature::Celsius(-273.15),   // 절대 영도
        Temperature::Celsius(-89.2),     // 기록된 최저 기온
        Temperature::Celsius(0.0),       // 물의 어는점
        Temperature::Celsius(20.0),      // 실내 온도
        Temperature::Celsius(37.0),      // 체온
        Temperature::Celsius(100.0),     // 물의 끓는점
    ];

    for temp in &reference_points {
        print_conversion_table(*temp);
        println!();
    }

    // ── 표 형식 변환 ──────────────────────────────────────────
    println!("── 2. 섭씨-화씨 변환표 ──\n");

    println!("{:>8} {:>10} {:>10}", "섭씨(°C)", "화씨(°F)", "켈빈(K)");
    println!("{}", "─".repeat(32));
    for c in (-20..=110).step_by(10) {
        let celsius = c as f64;
        let fahrenheit = celsius_to_fahrenheit(celsius);
        let kelvin = celsius + 273.15;
        let marker = match c {
            0 => " ← 어는점",
            100 => " ← 끓는점",
            37 => " ← 체온",
            _ => "",
        };
        println!("{:>8.0} {:>10.2} {:>10.2}{}", celsius, fahrenheit, kelvin, marker);
    }

    // ── 각 단위 입력 변환 ──────────────────────────────────────
    println!("\n── 3. 다양한 단위 입력 ──\n");

    print_conversion_table(Temperature::Fahrenheit(98.6));     // 체온
    println!();
    print_conversion_table(Temperature::Kelvin(0.0));           // 절대 영도
    println!();
    print_conversion_table(Temperature::Rankine(671.67));       // ~200°C in Rankine

    // ── 함수형 스타일 변환 ────────────────────────────────────
    println!("\n── 4. 함수형 파이프라인 ──");

    let temperatures_c = [-10.0, 0.0, 15.0, 20.0, 25.0, 30.0, 37.0, 40.0, 100.0];

    let comfortable: Vec<f64> = temperatures_c.iter()
        .copied()
        .filter(|&c| (18.0..=24.0).contains(&c))
        .collect();
    println!("쾌적한 온도 (18~24°C): {:?}", comfortable);

    let fahrenheit_list: Vec<(f64, f64)> = temperatures_c.iter()
        .map(|&c| (c, celsius_to_fahrenheit(c)))
        .collect();
    println!("\n(°C, °F) 쌍:");
    for (c, f) in &fahrenheit_list {
        println!("  ({:.1}°C, {:.1}°F)", c, f);
    }

    // ── 역변환 검증 ───────────────────────────────────────────
    println!("\n── 5. 역변환 정확도 검증 ──");

    for &c in &[-100.0f64, 0.0, 37.0, 100.0, 1000.0] {
        let f = celsius_to_fahrenheit(c);
        let back = fahrenheit_to_celsius(f);
        let error = (c - back).abs();
        println!("{}°C → {}°F → {}°C (오차: {:.2e})", c, f, back, error);
    }

    // ── 지구 행성들의 평균 온도 ───────────────────────────────
    println!("\n── 6. 태양계 행성 온도 ──\n");

    let planets = [
        ("수성", -180.0, 430.0),
        ("금성", 465.0,  465.0),
        ("지구",  15.0,   15.0),
        ("화성", -80.0,  -80.0),
        ("목성", -145.0, -145.0),
    ];

    println!("{:<8} {:>10} {:>10} {:>10}", "행성", "최저(°C)", "최고(°C)", "화씨(°F)");
    println!("{}", "─".repeat(44));
    for (planet, min, max) in &planets {
        println!("{:<8} {:>10.1} {:>10.1} {:>10.1}",
            planet, min, max,
            celsius_to_fahrenheit(*max)
        );
    }
}
