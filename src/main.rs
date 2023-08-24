mod api;

use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use nimb::{Config, AssetType};
use clap::{Parser, Subcommand, Args};
use api::find;

#[derive(Debug, Parser, Clone)]
#[command(name = "nimb")]
#[command(author = "hex.execute and rfwm")]
#[command(version = "0.1.0")]
#[command(about = "a cli minecraft modpack manager & launcher")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    Add(AddArgs),
}

#[derive(Debug, Args, Clone)]
struct AddArgs {
    name: String,
    save: String,
    r#type: Option<AssetType>
}

#[tokio::main]
async fn main() {
    let config = Config::load();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => {
            let raw_results = find(
                args.r#type.clone().unwrap_or(AssetType::FabricMod),
                "1.19.2".to_owned(),
                args.name.clone()
            );

            let string_results: Vec<String> = raw_results
                .await
                .iter()
                .map(|x| x.to_string())
                .collect();

             let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("pick the asset")
                .default(0)
                .items(&string_results[..])
                .interact()
                .unwrap();
        }
    }
}
