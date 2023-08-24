use std::{path::PathBuf, fs, fmt};

use serde::{Serialize, Deserialize};
use clap::ValueEnum;

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

    pub fn new_instance() {
        
    }

    fn default() -> Self {
        Self {}
    }

    fn read(path: &PathBuf) -> Self {
        let config_str = fs::read_to_string(path).expect("problem reading Config.toml");
        toml::from_str(&config_str).expect("problem parsing Config.toml")
    }

    fn write(path: &PathBuf, config: &Self) {
        let config_str = toml::to_string(config).expect("problem converting Config to string");
        fs::write(path, config_str).expect("problem writing Config.toml");
    }

    fn create_dir(config_dir: &PathBuf, name: &str) {
        let dir = config_dir.join(name);
        if !dir.exists() {
            fs::create_dir(&dir).expect(&format!("problem creating {} directory", name).as_str());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    Mod,
    ModPack,
    ResourcePack,
    Shader
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
            ProjectType::Mod => "mod",
            ProjectType::ModPack => "modpack",
            ProjectType::ResourcePack => "resourcepack",
            ProjectType::Shader => "shader"
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub slug: String,
    pub title: String,
    description: String,
    project_type: ProjectType,
    dependencies: Option<Vec<String>>
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
pub enum Loader {
    Fabric,
    Quilt,
    Forge,
    Plugin,
    DataPack
}

impl fmt::Display for Loader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
            Loader::Fabric => "fabric",
            Loader::Quilt => "quilt",
            Loader::Forge => "forge",
            Loader::Plugin => "plugin",
            Loader::DataPack => "datapack",
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ModrinthSearchResponse {
    pub hits: Vec<Project>,
    offset: i32,
    limit: u32,
    total_hits: u32
}
