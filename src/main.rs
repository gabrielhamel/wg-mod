use crate::config::Configs;

mod builder;
mod cli;
mod config;
mod dependency;
mod executable;
mod new;
mod sdk;
mod utils;

fn main() {
    Configs::load().expect("Unable to load wg-mod configs");

    match cli::run() {
        | Err(err) => eprintln!("{}", err.to_string()),
        | _ => (),
    };
}
