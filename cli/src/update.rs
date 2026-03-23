use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use crate::commands::utils::get_config_dir;
use tracing::debug;

const CACHE_FILE: &str = ".update_cache";
const GITHUB_API_URL: &str = "https://api.github.com/repos/Jcd1230/audiobook-downloader/releases/latest";

#[derive(Debug, Serialize, Deserialize)]
struct UpdateCache {
    last_check: u64,
    latest_version: String,
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

pub async fn check_for_update() -> Option<String> {
    let cache_path = get_config_dir().join(CACHE_FILE);
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    if let Ok(data) = std::fs::read_to_string(&cache_path) {
        if let Ok(cache) = serde_json::from_str::<UpdateCache>(&data) {
            if now - cache.last_check < 86400 { // 24 hours
                return is_newer(&cache.latest_version).then(|| cache.latest_version);
            }
        }
    }

    debug!("Checking for updates on GitHub...");
    let client = reqwest::Client::builder()
        .user_agent("audiobook-downloader-update-checker")
        .timeout(Duration::from_secs(2))
        .build()
        .ok()?;

    let resp = client.get(GITHUB_API_URL).send().await.ok()?;
    if let Ok(release) = resp.json::<GitHubRelease>().await {
        let latest = release.tag_name.trim_start_matches('v').to_string();
        let cache = UpdateCache {
            last_check: now,
            latest_version: latest.clone(),
        };
        let _ = serde_json::to_string(&cache).map(|s| std::fs::write(&cache_path, s));
        
        if is_newer(&latest) {
            return Some(latest);
        }
    }

    None
}

fn is_newer(latest: &str) -> bool {
    let current = env!("CARGO_PKG_VERSION");
    if let (Ok(v_latest), Ok(v_current)) = (semver::Version::parse(latest), semver::Version::parse(current)) {
        v_latest > v_current
    } else {
        false
    }
}
