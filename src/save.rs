use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
    launch_cmd: String
}
