use super::registry::SidecarEntry;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

/// Spawns all registered sidecars. Each sidecar receives its port via the
/// SIDECAR_PORT environment variable so it can bind to 127.0.0.1:<port>.
pub async fn spawn_all(app: &AppHandle, sidecars: &[SidecarEntry]) {
    for entry in sidecars {
        if let Err(e) = spawn_one(app, entry).await {
            eprintln!("[sidecar] failed to spawn {}: {e}", entry.id);
        }
    }
}

async fn spawn_one(app: &AppHandle, entry: &SidecarEntry) -> Result<(), String> {
    let cmd = app
        .shell()
        .sidecar(entry.binary)
        .map_err(|e| e.to_string())?
        .env("SIDECAR_PORT", entry.port.to_string());

    let (_rx, _child) = cmd.spawn().map_err(|e| e.to_string())?;

    // Child handle is intentionally leaked here. The OS will reclaim the
    // processes when Tauri exits. For a production app, store _child in
    // a global Arc<Mutex<>> and call child.kill() on app exit.
    println!("[sidecar] spawned {} on port {}", entry.id, entry.port);
    Ok(())
}
