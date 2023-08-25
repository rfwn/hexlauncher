pub mod config;
pub mod instance;

use config::Config;
use instance::Instance;

pub struct Launcher {
    config: Config,
    pub instances: Vec<Instance>
}

impl Launcher {
    pub fn load() -> Self {
        Self {
            config: Config::load(),
            instances: vec![]
        }
    }
}
