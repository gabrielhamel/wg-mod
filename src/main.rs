mod config;
mod new;
mod sdk;
mod utils;

use clap::Command;
use sdk::conda::Conda;

fn root() -> Command {
    Command::new("wg-mod")
        .version("0.1.0")
        .author("Gabriel Hamel <gabriel.hamel.pro@gmail.com>")
        .about("Provides tools for wargaming modding")
        .subcommand_required(true)
        .subcommand(new::command())
}
#[tokio::main]
async fn main() {
    // let matches = root().get_matches();

    // match matches.subcommand() {
    //     | Some(("new", args)) => new::execute(args),
    //     | Some((_, _)) => panic!("Not implemented"),
    //     | None => panic!("No command provided"),
    // }

    let conda = Conda::default().unwrap();
    conda.install().await.unwrap();

    let version = conda.version().expect("");

    println!("version: {}", version);
}
