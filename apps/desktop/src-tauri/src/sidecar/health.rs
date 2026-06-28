use super::registry::{SidecarEntry, SIDECARS};
use serde::Serialize;
use std::collections::HashMap;
use tauri::AppHandle;

#[derive(Serialize)]
pub struct HealthResult {
    pub id: String,
    pub port: u16,
    pub ok: bool,
}

/// Polls GET /health on every registered sidecar and returns the results.
/// Language-agnostic: works for any sidecar that implements the shared contract.
pub async fn check_all(_app: &AppHandle) -> Vec<HealthResult> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .unwrap_or_default();

    let mut results = Vec::with_capacity(SIDECARS.len());

    for entry in SIDECARS {
        let ok = probe(&client, entry).await;
        results.push(HealthResult {
            id: entry.id.to_string(),
            port: entry.port,
            ok,
        });
    }

    results
}

async fn probe(client: &reqwest::Client, entry: &SidecarEntry) -> bool {
    let url = format!("http://127.0.0.1:{}{}", entry.port, entry.health_path);
    match client.get(&url).send().await {
        Ok(res) => res.status().is_success(),
        Err(_) => false,
    }
}

#[allow(dead_code)]
pub async fn wait_until_ready(
    _app: &AppHandle,
    timeout_secs: u64,
) -> HashMap<String, bool> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .unwrap_or_default();

    let deadline = std::time::Instant::now()
        + std::time::Duration::from_secs(timeout_secs);

    let mut ready: HashMap<String, bool> = SIDECARS
        .iter()
        .map(|e| (e.id.to_string(), false))
        .collect();

    while std::time::Instant::now() < deadline {
        let all_ok = {
            let mut all = true;
            for entry in SIDECARS {
                if !ready[entry.id] {
                    let ok = probe(&client, entry).await;
                    if ok {
                        ready.insert(entry.id.to_string(), true);
                    } else {
                        all = false;
                    }
                }
            }
            all
        };

        if all_ok {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }

    ready
}
