use crate::config::Configs;

mod builder;
mod cli;
mod config;
mod new;
mod sdk;
mod utils;

fn main() {
    Configs::load().expect("");

    match cli::run() {
        | Err(err) => eprintln!("{}", err.to_string()),
        | _ => (),
    };
}
