// Example 12: tokio::sync::watch — share the latest value across tasks
// Run: cargo run --example 12_watch_channel

use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
struct Config {
    theme: String,
    page_size: u32,
}

#[tokio::main]
async fn main() {
    println!("=== tokio::sync::watch ===\n");

    let initial = Config {
        theme: "light".into(),
        page_size: 20,
    };

    // watch: always holds the latest value; receivers get notified on change
    let (tx, rx) = tokio::sync::watch::channel(initial);
    let mut worker_rx = rx.clone();

    let worker = tokio::spawn(async move {
        loop {
            worker_rx.changed().await.unwrap();
            let cfg = worker_rx.borrow().clone();
            println!("  worker reloaded config: {:?}", cfg);
        }
    });

    tokio::time::sleep(Duration::from_millis(30)).await;
    tx.send(Config {
        theme: "dark".into(),
        page_size: 20,
    })
    .unwrap();

    tokio::time::sleep(Duration::from_millis(30)).await;
    tx.send(Config {
        theme: "dark".into(),
        page_size: 50,
    })
    .unwrap();

    tokio::time::sleep(Duration::from_millis(30)).await;
    worker.abort();

    let latest = rx.borrow().clone();
    println!("\nfinal config: {:?}", latest);
    println!("\nwatch is ideal for config, feature flags, and shutdown signals.");
}
