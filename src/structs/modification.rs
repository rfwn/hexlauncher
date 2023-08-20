use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    name: String,
    description: String,
    source: String
}
