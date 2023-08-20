use core::fmt;

use clap::Parser;

use crate::api::find;

mod structs;
mod api;

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum AssetType {
    Mod,
    Plugin,
    DataPack,
    Shader,
    ResourcePack
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssetType::Mod => write!(f, "mod"),
            AssetType::Plugin => write!(f, "plugin"),
            AssetType::DataPack => write!(f, "datapack"),
            AssetType::Shader => write!(f, "shader"),
            AssetType::ResourcePack => write!(f, "resourcepack"),
        }
    }
}

#[derive(Debug, Parser, Clone)]
#[command(name = "nimb")]
#[command(author = "hex.execute and rfwm")]
#[command(version = "0.1.0")]
#[command(about = "a cli minecraft modpack manager & launcher")]
struct Cli {
    #[arg(short('s'), long, value_name = "SAVE")]
    select: String,

    #[arg(short('t'), long, default_value = "mod", value_name = "ASSET")]
    r#type: AssetType,

    #[arg(short('a'), long, value_name = "ASSET NAME", requires = "select")]
    add: String
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    dbg!(&cli);

    dbg!(find(cli.r#type, "1.19.2".to_owned(), cli.add).await);
}
