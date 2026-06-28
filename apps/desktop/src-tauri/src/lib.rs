mod sidecar;

use sidecar::registry::SIDECARS;

#[tauri::command]
async fn get_sidecar_status(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let statuses = sidecar::health::check_all(&app).await;
    Ok(serde_json::to_value(statuses).map_err(|e| e.to_string())?)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                sidecar::spawn::spawn_all(&handle, &SIDECARS).await;
            });
            Ok(())
        })
        .on_window_event(|_window, event| {
            // send SIGTERM to sidecars when the last window closes
            if let tauri::WindowEvent::Destroyed = event {
                // child processes are dropped automatically; OS will clean up
            }
        })
        .invoke_handler(tauri::generate_handler![get_sidecar_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
