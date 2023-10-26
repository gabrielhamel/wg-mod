mod builder;
mod cli;
mod config;
mod new;
mod sdk;
mod utils;

fn main() {
    match cli::run() {
        | Err(err) => eprintln!("{}", err.to_string()),
        | _ => (),
    }
}
