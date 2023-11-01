use crate::sdk::game_sources::GameSources;
use std::path::PathBuf;

mod builder;
mod cli;
mod config;
mod new;
mod sdk;
mod utils;

fn main() {
    let path = PathBuf::from("/Users/gabriel/.wg-mod/wot-src");
    let gs = GameSources::new(&path).expect("");
    println!("{:?}", gs.list_branches().expect(""));

    match cli::run() {
        | Err(err) => eprintln!("{}", err.to_string()),
        | _ => (),
    };
}
