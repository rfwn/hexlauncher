pub mod launcher;

use std::fmt;

use serde::{Serialize, Deserialize};
use clap::ValueEnum;

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
