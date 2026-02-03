use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub copy_to_clipboard: bool,
    pub auto_open: bool,
    pub save_locally: bool,
    pub save_path: Option<String>,
    pub open_with_program: String, // "default" or path to program
}

impl Default for Config {
    fn default() -> Self {
        Self {
            copy_to_clipboard: true,
            auto_open: true,
            save_locally: true,
            save_path: None, // None means use default Pictures/Screenshots
            open_with_program: "default".to_string(),
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf, String> {
    let mut path = dirs::config_dir().ok_or("Could not find config directory")?;
    path.push("opencap");
    std::fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create config dir: {e}"))?;
    Ok(path)
}

pub fn get_config_path() -> Result<PathBuf, String> {
    Ok(get_config_dir()?.join("config.json"))
}

pub fn load_config() -> Config {
    match get_config_path() {
        Ok(path) => {
            if path.exists() {
                match std::fs::read_to_string(&path) {
                    Ok(contents) => {
                        match serde_json::from_str(&contents) {
                            Ok(config) => config,
                            Err(e) => {
                                log::warn!("Failed to parse config, using defaults: {e}");
                                Config::default()
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to read config file, using defaults: {e}");
                        Config::default()
                    }
                }
            } else {
                Config::default()
            }
        }
        Err(e) => {
            log::warn!("Failed to get config path, using defaults: {e}");
            Config::default()
        }
    }
}

pub fn save_config(config: &Config) -> Result<(), String> {
    let path = get_config_path()?;
    let contents = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {e}"))?;
    std::fs::write(&path, contents)
        .map_err(|e| format!("Failed to write config file: {e}"))?;
    Ok(())
}
