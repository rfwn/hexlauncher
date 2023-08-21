mod api;

use nimb::{Config, AssetType};
use clap::Parser;
use api::find;

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
    let config = Config::load();

    let cli = Cli::parse();

    dbg!(&cli);

    dbg!(find(cli.r#type, "1.19.2".to_owned(), cli.add).await);
}
