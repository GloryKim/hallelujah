// Example 15: JoinSet — manage a dynamic collection of tasks
// Run: cargo run --example 15_join_set

use std::time::Duration;
use tokio::task::JoinSet;

async fn download(id: u32, fail: bool) -> Result<u32, &'static str> {
    tokio::time::sleep(Duration::from_millis(50 * id as u64)).await;
    if fail {
        Err("network error")
    } else {
        Ok(id * 100)
    }
}

#[tokio::main]
async fn main() {
    println!("=== JoinSet ===\n");

    let mut set = JoinSet::new();

    for id in 1..=5 {
        let fail = id == 3;
        set.spawn(async move { download(id, fail).await });
    }

    println!("-- results as tasks finish --");
    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok(bytes)) => println!("  ok: {} bytes", bytes),
            Ok(Err(err)) => println!("  task error: {}", err),
            Err(join_err) => println!("  join error: {}", join_err),
        }
    }

    println!("\n-- spawn more tasks into the same set --");
    set.spawn(async { download(10, false).await });
    set.spawn(async { download(11, false).await });

    while let Some(result) = set.join_next().await {
        if let Ok(Ok(bytes)) = result {
            println!("  ok: {} bytes", bytes);
        }
    }

    println!("\nJoinSet is useful when the number of tasks is not known upfront.");
}
