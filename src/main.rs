use crate::config::Configs;
use crate::sdk::conda::environment::PythonEnvironment;
use crate::sdk::conda::Conda;

mod builder;
mod cli;
mod config;
mod new;
mod sdk;
mod utils;

async fn get_conda() -> Conda {
    let config = Configs::load().expect("");

    let conda_path = config.wg_mod_home.join("conda");
    let conda = Conda::from(conda_path);

    if !conda.is_installed().expect("") {
        println!("Installing conda...");
        conda.install().await.expect("");
    }

    conda
}

fn get_python_2_env(conda: Conda) -> PythonEnvironment {
    if !conda.has_environment("wg-mod") {
        println!("Create conda env...");
        conda.create_environment("wg-mod", "2").expect("");
    }

    conda.get_environment("wg-mod")
}

#[tokio::main]
async fn main() {
    let python_environment = get_python_2_env(get_conda().await);

    println!("{:?}", python_environment.version());

    match cli::run() {
        | Err(err) => eprintln!("{:?}", err),
        | _ => (),
    }
}
