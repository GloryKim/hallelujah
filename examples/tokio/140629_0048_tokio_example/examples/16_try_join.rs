// Example 16: try_join! — fail fast when any Future returns Err
// Run: cargo run --example 16_try_join

use std::time::Duration;

async fn fetch_ok(label: &str, ms: u64) -> Result<&str, &'static str> {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    Ok(label)
}

async fn fetch_fail(label: &'static str, ms: u64) -> Result<&'static str, &'static str> {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    Err(label)
}

#[tokio::main]
async fn main() {
    println!("=== try_join! ===\n");

    let all_ok = tokio::try_join!(
        fetch_ok("users", 80),
        fetch_ok("orders", 60),
        fetch_ok("inventory", 40),
    );
    println!("all ok: {:?}", all_ok);

    let one_fails = tokio::try_join!(
        fetch_ok("users", 80),
        fetch_fail("payments", 30),
        fetch_ok("inventory", 100),
    );
    println!("one fails: {:?}", one_fails);

    println!("\ntry_join! returns Err as soon as any branch fails.");
    println!("Unlike join!, it short-circuits instead of waiting for every branch.");
}
