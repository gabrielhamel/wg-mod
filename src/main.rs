use crate::config::Configs;
use crate::sdk::conda::environment::CondaEnvironment;
use crate::sdk::conda::Conda;

mod builder;
mod cli;
mod config;
mod new;
mod sdk;
mod utils;

fn get_conda() -> Conda {
    let config = Configs::load().expect("");

    let conda_path = config.wg_mod_home.join("conda");
    let conda = Conda::from(conda_path);

    if !conda.is_installed().expect("") {
        println!("Installing conda...");
        conda.install().expect("");
    }

    conda
}

fn get_python_2_environment(conda: Conda) -> CondaEnvironment {
    if !conda.has_environment("wg-mod") {
        println!("Create conda env...");
        conda.create_environment("wg-mod", "2").expect("");
    }

    conda.get_environment("wg-mod")
}

fn main() {
    let conda = get_conda();
    let conda_environment = get_python_2_environment(conda);

    // let python_builder = PythonBuilder::from(conda_environment);
    // python_builder.compile_all(PathBuf::from("/Users/gabriel/Development/wot/WoT-Replay-Timeline/scripts")).expect("");

    match cli::run() {
        | Err(err) => eprintln!("{:?}", err),
        | _ => (),
    }
}
