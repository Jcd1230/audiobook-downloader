use serde::{Deserialize, Serialize};
use crate::commands::utils::get_config_dir;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub library_path: Option<String>,
    pub filename_template: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let path = get_config_dir().join("config.json");
        if let Ok(data) = std::fs::read_to_string(path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = get_config_dir().join("config.json");
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}
