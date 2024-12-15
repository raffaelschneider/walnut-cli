use directories::BaseDirs;
use serde::Deserialize;
use std::fs;
use std::path::{PathBuf};

#[derive(Deserialize, Default)]
pub struct Config {
    // Add fields as needed
    // Example:
    // pub default_editor: Option<String>,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = config_dir().join("config.toml");
    if config_path.exists() {
        let data = fs::read_to_string(config_path)?;
        let cfg: Config = toml::from_str(&data)?;
        Ok(cfg)
    } else {
        Ok(Config::default())
    }
}

pub fn config_dir() -> PathBuf {
    let base = if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        PathBuf::from(xdg)
    } else {
        BaseDirs::new().unwrap().config_dir().to_path_buf()
    };
    base.join("walnut")
}

pub fn data_dir() -> PathBuf {
    let base = if let Some(xdg) = std::env::var_os("XDG_DATA_HOME") {
        PathBuf::from(xdg)
    } else {
        directories::BaseDirs::new().unwrap().data_dir().to_path_buf()
    };
    base
}

