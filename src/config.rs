use std::{path::PathBuf, fs::{self, File}};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config;

impl Config {
    pub fn load() -> Self {
        let mut config = Config::default();
        let config_dir = dirs::config_dir().expect("problem determining config directory");
        let config_path = config_dir.join("nimb/Config.toml");

        if config_path.exists() { config = Config::read(&config_path); }
        else { Config::write(&config_path, &config) }

        config
    }

    fn read(path: &PathBuf) -> Self {
        let config_str = fs::read_to_string(path).expect("problem reading Config.toml");
        toml::from_str(&config_str).expect("problem parsing Config.toml")
    }

    fn write(path: &PathBuf, config: &Self) {
        let config_str = toml::to_string(config).expect("problem converting Config to string");
        fs::write(path, config_str).expect("problem writing Config.toml");
    }

    pub fn default() -> Self {
        Self
    }
}
