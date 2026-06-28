// Example 27: Async file processor — concurrent I/O with tokio::fs
// Run: cargo run --example 27_async_file_processor
//
// tokio::fs wraps blocking filesystem calls on the blocking thread pool.
// Combine with Semaphore to limit concurrent disk operations.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tokio::fs;
use tokio::sync::Semaphore;

#[derive(Debug)]
struct FileStats {
    path: PathBuf,
    lines: usize,
    words: usize,
    bytes: u64,
}

async fn analyze_file(path: PathBuf, limiter: Arc<Semaphore>) -> std::io::Result<FileStats> {
    let _permit = limiter.acquire().await.unwrap();

    let content = fs::read_to_string(&path).await?;
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let bytes = content.len() as u64;

    // Simulate heavier processing
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    Ok(FileStats {
        path,
        lines,
        words,
        bytes,
    })
}

async fn setup_test_files(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    fs::create_dir_all(dir).await?;

    let files = [
        ("alpha.txt", "hello world\nfoo bar baz\n"),
        ("beta.txt", "tokio async fs\ncount words here\nthird line\n"),
        ("gamma.txt", "one two three four five\n"),
    ];

    let mut paths = Vec::new();
    for (name, content) in files {
        let path = dir.join(name);
        fs::write(&path, content).await?;
        paths.push(path);
    }
    Ok(paths)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("=== Async file processor ===\n");

    let temp_dir = std::env::temp_dir().join("tokio_example_27");
    let paths = setup_test_files(&temp_dir).await?;
    println!("created {} test files in {}\n", paths.len(), temp_dir.display());

    let limiter = Arc::new(Semaphore::new(2)); // max 2 concurrent file reads
    let start = Instant::now();

    let mut handles = Vec::new();
    for path in paths {
        let limiter = Arc::clone(&limiter);
        handles.push(tokio::spawn(async move { analyze_file(path, limiter).await }));
    }

    let mut total_bytes = 0u64;
    let mut total_words = 0usize;

    for handle in handles {
        let stats = handle.await.unwrap()?;
        println!(
            "  {} — {} lines, {} words, {} bytes",
            stats.path.file_name().unwrap().to_string_lossy(),
            stats.lines,
            stats.words,
            stats.bytes
        );
        total_bytes += stats.bytes;
        total_words += stats.words;
    }

    println!("\nprocessed all files in {:?}", start.elapsed());
    println!("totals: {total_words} words, {total_bytes} bytes");

    fs::remove_dir_all(&temp_dir).await.ok();

    println!("\ntokio::fs is async-friendly but uses blocking threads under the hood.");
    println!("Limit concurrency to avoid saturating the blocking pool.");
    Ok(())
}
