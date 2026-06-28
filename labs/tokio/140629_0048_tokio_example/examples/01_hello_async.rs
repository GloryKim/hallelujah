// Example 01: async/await basics — #[tokio::main] and Future
// Run: cargo run --example 01_hello_async

async fn greet(name: &str) {
    println!("Hello, {}!", name);
}

async fn add(a: i32, b: i32) -> i32 {
    a + b
}

async fn fetch_message(id: u32) -> String {
    // Simulate I/O with a short delay instead of real network I/O
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    format!("message #{}", id)
}

#[tokio::main]
async fn main() {
    println!("=== async/await basics ===\n");

    // Await async functions with .await
    greet("Tokio").await;
    greet("Rust").await;

    let sum = add(10, 32).await;
    println!("10 + 32 = {}", sum);

    // Run multiple async calls sequentially
    println!("\n-- sequential await --");
    for id in 1..=3 {
        let msg = fetch_message(id).await;
        println!("  {}", msg);
    }

    println!("\nasync fn returns a Future.");
    println!("The Future does not run until you .await it.");
}
