use crate::config::Configs;
use crate::dependency::store::DependencyStore;
use crate::sdk::conda2::CondaV2;

mod builder;
mod cli;
mod config;
mod dependency;
mod executable;
mod new;
mod sdk;
mod utils;

fn main() {
    let mut store = DependencyStore::default();
    store.register("conda", Box::new(CondaV2::default()));
    let conda = store.get("conda").unwrap();

    Configs::load().expect("Unable to load wg-mod configs");

    match cli::run() {
        | Err(err) => eprintln!("{}", err.to_string()),
        | _ => (),
    };
}
