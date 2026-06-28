// Example 04: tokio::join! — wait for multiple Futures concurrently
// Run: cargo run --example 04_join_macro

use std::time::Instant;

async fn fetch_user(id: u32) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    format!("user-{}", id)
}

async fn fetch_posts(user: &str) -> Vec<String> {
    tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    vec![
        format!("{} post 1", user),
        format!("{} post 2", user),
    ]
}

async fn fetch_avatar(user: &str) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    format!("avatar://{}", user)
}

#[tokio::main]
async fn main() {
    println!("=== tokio::join! ===\n");

    let start = Instant::now();

    // join!: wait until all Futures complete, return results as a tuple
    let (user, posts, avatar) = tokio::join!(
        fetch_user(42),
        fetch_posts("user-42"),
        fetch_avatar("user-42"),
    );

    println!("user: {}", user);
    println!("posts: {:?}", posts);
    println!("avatar: {}", avatar);
    println!("elapsed: {:?}\n", start.elapsed());

    // join! with spawn handles
    println!("-- spawn + join! --");
    let start = Instant::now();
    let h1 = tokio::spawn(fetch_user(1));
    let h2 = tokio::spawn(fetch_user(2));
    let h3 = tokio::spawn(fetch_user(3));

    let (r1, r2, r3) = tokio::join!(h1, h2, h3);
    println!("results: {}, {}, {}", r1.unwrap(), r2.unwrap(), r3.unwrap());
    println!("elapsed: {:?} (parallel)", start.elapsed());

    println!("\ntokio::join! waits only as long as the slowest Future.");
}
