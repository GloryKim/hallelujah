// Example 29: Async process pipeline — spawn subprocess, pipe stdin/stdout
// Run: cargo run --example 29_async_process_pipeline
//
// tokio::process::Command is the async equivalent of std::process::Command.
// Use it to run CLI tools, shell scripts, or worker binaries from async code.

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

#[cfg(unix)]
fn shell_command(script: &str) -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(script);
    cmd
}

#[cfg(windows)]
fn shell_command(script: &str) -> Command {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg(script);
    cmd
}

/// Run a shell pipeline and collect stdout lines.
async fn run_pipeline(script: &str) -> std::io::Result<Vec<String>> {
    let output = shell_command(script)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?
        .wait_with_output()
        .await?;

    if !output.status.success() {
        eprintln!(
            "  pipeline failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(String::from).collect())
}

/// Interactive: write to stdin, read lines from stdout.
async fn run_interactive(input: &str) -> std::io::Result<Vec<String>> {
    let mut child = shell_command("while read line; do echo \"processed: $line\"; done")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");

    let mut writer = stdin;
    writer.write_all(input.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    drop(writer); // close stdin so the shell loop exits

    let mut reader = BufReader::new(stdout);
    let mut lines = Vec::new();
    let mut line = String::new();

    loop {
        line.clear();
        let n = reader.read_line(&mut line).await?;
        if n == 0 {
            break;
        }
        lines.push(line.trim().to_string());
    }

    let status = child.wait().await?;
    println!("  exit status: {status}");
    Ok(lines)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("=== Async process pipeline ===\n");

    // Batch pipeline: run a shell one-liner
    println!("-- batch pipeline --");
    let lines = run_pipeline("echo 'alpha beta gamma' | tr ' ' '\\n' | sort").await?;
    for line in &lines {
        println!("  {line}");
    }

    // Interactive pipeline: stream data to a subprocess
    println!("\n-- interactive pipeline --");
    let processed = run_interactive("hello from tokio").await?;
    for line in &processed {
        println!("  {line}");
    }

    println!("\ntokio::process::Command integrates with async I/O on pipes.");
    println!("Always set stdout/stderr to piped() if you need to read them.");
    Ok(())
}
