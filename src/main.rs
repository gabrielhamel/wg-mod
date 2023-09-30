mod cli;
mod config;
mod new;
mod sdk;
mod utils;

#[tokio::main]
async fn main() {
    let conda = sdk::conda::Conda::default().expect("");
    conda.install_if_not_installed().await.expect("");

    match cli::run() {
        | Err(err) => eprintln!("{:?}", err),
        | _ => (),
    }
}
