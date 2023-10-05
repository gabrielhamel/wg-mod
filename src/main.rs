use crate::config::Configs;
use crate::sdk::conda::Conda;

mod cli;
mod config;
mod new;
mod sdk;
mod utils;

#[tokio::main]
async fn main() {
    let config = Configs::load().expect("");

    let conda_path = config.wg_mod_home.join("conda");
    let conda = Conda::from(conda_path);

    if !conda.is_installed().expect("") {
        println!("Installing conda...");
        conda.install().await.expect("");
    }

    println!("Conda installed version: '{}'", conda.version().expect(""));

    if !conda.has_environment("wg-mod") {
        println!("Create conda env...");
        conda.create_environment("wg-mod", "2").expect("");
    }

    let env = conda.get_environment("wg-mod");

    println!("Python installed version '{}'", env.version().expect(""));

    match cli::run() {
        | Err(err) => eprintln!("{:?}", err),
        | _ => (),
    }
}
