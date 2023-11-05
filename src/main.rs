mod builder;
mod cli;
mod config;
mod new;
mod sdk;
mod utils;

use crate::config::Configs;
use crate::sdk::game_sources::GameSources;

fn main() {
    let config = Configs::load().expect("Failed to load config");
    let game_sources_path = config.wg_mod_home.join("wot-src");
    GameSources::load(&game_sources_path).expect("");

    match cli::run() {
        | Err(err) => eprintln!("{}", err.to_string()),
        | _ => (),
    };
}
