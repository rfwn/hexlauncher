mod api;

use api::{ApiWrapper, ModrinthWrapper};
use clap::{Args, Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use nimb::{launcher::{config::Config, Launcher, instance::Instance}, Loader, ProjectType};

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
    Create(CreateArgs),
}

#[derive(Debug, Args, Clone)]
struct AddArgs {
    name: String,
    save: String,
    r#type: Option<Loader>,
}

#[derive(Debug, Args, Clone)]
struct CreateArgs {
    name: String,
    loader: Loader,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let launcher = Launcher::load();

    dbg!(launcher.instances);

    match &cli.command {
        Commands::Add(args) => {
            let raw_results = ModrinthWrapper::search_projects(
                args.name.clone(),
                String::from("1.19.2"),
                ProjectType::Mod,
                Some(args.r#type.clone().unwrap_or(Loader::Fabric)),
            ).await;

            if raw_results.is_empty() {
                eprintln!("no mods found");
                return;
            }

            let string_results: Vec<String> = raw_results
                .iter()
                .map(|x| format!("'{}' ({})", x.title, x.slug))
                .collect();

            let _selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("pick the asset")
                .default(0)
                .items(&string_results[..])
                .interact()
                .unwrap();
        }
        Commands::Create(_args) => {
            let _config = Config::load();
        }
    }
}
