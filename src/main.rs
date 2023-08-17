use config::Config;

mod config;
mod save;

fn main() {
    let config = Config::load();
}
