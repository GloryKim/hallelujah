// Example 03: tokio::time::sleep — async waiting
// Run: cargo run --example 03_sleep_and_delay

use std::time::{Duration, Instant};

async fn countdown(from: u32) {
    for n in (1..=from).rev() {
        println!("  {}", n);
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
    println!("  liftoff!");
}

#[tokio::main]
async fn main() {
    println!("=== tokio::time::sleep ===\n");

    // sleep does not block the thread — other tasks can run
    println!("-- countdown --");
    countdown(3).await;

    println!("\n-- sleep vs thread::sleep --");
    let start = Instant::now();

    let h1 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(200)).await;
        "task A"
    });
    let h2 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(200)).await;
        "task B"
    });

    let (a, b) = tokio::join!(h1, h2);
    let elapsed = start.elapsed();

    println!("  {} + {} done", a.unwrap(), b.unwrap());
    println!("  elapsed: {:?} (~400ms sequential, ~200ms parallel)", elapsed);

    println!("\ntokio::time::sleep yields the CPU to other tasks while awaiting.");
}
