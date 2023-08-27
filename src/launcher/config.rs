use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub directory: PathBuf,
}

impl Config {
    pub fn load() -> Self {
        let mut config = Config::default();

        if !config.directory.exists() {
            fs::create_dir(&config.directory).expect("problem creating config directory");
        }

        let config_path = &config.directory.join("Config.toml");

        if config_path.exists() {
            config = Config::read(&config_path);
        } else {
            Config::write(&config_path, &config)
        }

        config
    }

    fn default() -> Self {
        let global_config_dir =
            dirs::config_dir().expect("problem determining global config directory");

        Self {
            directory: global_config_dir.join("nimb"),
        }
    }

    fn read(path: &PathBuf) -> Self {
        let config_str = fs::read_to_string(path).expect("problem reading Config.toml");
        toml::from_str(&config_str).expect("problem parsing Config.toml")
    }

    fn write(path: &PathBuf, config: &Self) {
        let config_str = toml::to_string(config).expect("problem converting Config to string");
        fs::write(path, config_str).expect("problem writing Config.toml");
    }
}
