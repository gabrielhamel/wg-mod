mod cli;
mod config;
mod new;
mod sdk;
mod utils;

#[tokio::main]
async fn main() {
    match cli::run() {
        | Err(err) => eprintln!("{:?}", err),
        | _ => (),
    }
}
