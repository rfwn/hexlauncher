use std::{path::PathBuf, fs, fmt};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

impl Config {
    pub fn load() -> Self {
        let mut config = Config::default();
        let global_config_dir = dirs::config_dir().expect("problem determining global config directory");

        let config_dir = global_config_dir.join("nimb");
        if !config_dir.exists() {
            fs::create_dir(&config_dir).expect("problem creating config directory"); 
        }

        Config::create_dir(&config_dir, "saves");
        Config::create_dir(&config_dir, "instances");


        let config_path = config_dir.join("Config.toml");

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
        Self {}
    }

    pub fn create_dir(config_dir: &PathBuf, name: &str) {
        let dir = config_dir.join(name);
        if !dir.exists() {
            fs::create_dir(&dir).expect(&format!("problem creating {} directory", name).as_str());
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    name: String,
    description: String,
    source: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Loader {
    Fabric,
    Quilt,
    Forge
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Save {
    loader: Loader,
    version: String,
    options: String,
    instance_dir: PathBuf,
    launch_cmd: String,
    modifications: Vec<Mod>
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    slug: String,
    title: String,
    description: String,
    versions: Vec<String>,
    dependencies: Option<Vec<String>>
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}' ({})", self.title, self.slug)
    }
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum AssetType {
    FabricMod,
    ForgeMod,
    QuiltMod,
    Plugin,
    DataPack,
    Shader,
    ResourcePack
}
