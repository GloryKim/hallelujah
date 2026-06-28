// Example 13: tokio::sync::RwLock — many readers, one writer
// Run: cargo run --example 13_rwlock_shared_cache

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type Cache = Arc<RwLock<HashMap<String, String>>>;

async fn read_cache(cache: Cache, key: &str, reader: &str) {
    let guard = cache.read().await;
    let value = guard.get(key).cloned().unwrap_or_else(|| "missing".into());
    println!("  {} read {} -> {}", reader, key, value);
}

async fn write_cache(cache: Cache, key: &str, value: &str) {
    let mut guard = cache.write().await;
    guard.insert(key.into(), value.into());
    println!("  writer updated {} = {}", key, value);
}

#[tokio::main]
async fn main() {
    println!("=== tokio::sync::RwLock ===\n");

    let cache: Cache = Arc::new(RwLock::new(HashMap::new()));

    write_cache(Arc::clone(&cache), "user:1", "Alice").await;

    let mut handles = Vec::new();
    for reader in ["api", "worker", "cache-warmer"] {
        let cache = Arc::clone(&cache);
        handles.push(tokio::spawn(async move {
            read_cache(cache, "user:1", reader).await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    write_cache(Arc::clone(&cache), "user:1", "Alice (admin)").await;
    read_cache(cache, "user:1", "api").await;

    println!("\nRwLock allows concurrent reads but exclusive writes.");
    println!("Use it when reads dominate and writes are infrequent.");
}
