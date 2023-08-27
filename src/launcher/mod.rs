pub mod config;
pub mod instance;

use std::{fs, path::PathBuf};

use config::Config;
use instance::Instance;

pub struct Launcher {
    pub config: Config,
    pub instances: Vec<Instance>,
}

impl Launcher {
    pub fn load() -> Self {
        let config = Config::load();

        let mut launcher = Self {
            config,
            instances: vec![],
        };

        let instances = launcher.ensure_directory(PathBuf::from("instances"));

        for instance_path in fs::read_dir(&instances).unwrap() {
            let instance_path = instance_path.unwrap();

            let instance: Instance = ron::from_str(
                &fs::read_to_string(
                    &instances
                        .join(instance_path.file_name())
                        .join("instance.ron"),
                )
                .unwrap()[..],
            )
            .unwrap();

            launcher.instances.push(instance);
        }

        launcher
    }

    pub fn ensure_directory(&self, path: PathBuf) -> PathBuf {
        let directory = &self.config.directory.join(path);
        if !directory.exists() {
            fs::create_dir(directory).unwrap();
        }

        directory.clone()
    }
}
