use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    
}

impl Config {
    pub fn new(path: String) -> Self {
        toml::from_str(
            &std::fs::read_to_string(path).expect("problem reading Config.toml")
        ).expect("problem casting Config.toml to Config")
    }
}
