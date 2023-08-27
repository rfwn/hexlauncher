use std::{path::PathBuf, fs};

use serde::{Serialize, Deserialize};
use crate::{Project, Loader};

use super::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
    title: String,
    slug: String,
    directory: PathBuf,
    mods: Vec<Project>,
    loader: Loader
}

impl Instance {
    pub fn new(config: &Config, title: String, slug: String, loader: Loader) -> Self {
        let instance = Self {
            title,
            slug: slug.clone(),
            directory: config.directory.join("instances").join(slug),
            mods: vec![],
            loader
        };

        fs::create_dir(&instance.directory).unwrap();
        fs::write(&instance.directory.join("instance.ron"), ron::to_string(&instance).unwrap()).unwrap();

        instance
    }

    pub async fn launch() {
        // use a result
    }
}
