mod config;
mod doctor;
mod new;

use clap::Command;

fn root() -> Command {
    Command::new("wg-mod")
        .version("0.1.0")
        .author("Gabriel Hamel <gabriel.hamel.pro@gmail.com>")
        .about("Provides tools for wargaming modding")
        .subcommand_required(true)
        .subcommand(doctor::command())
        .subcommand(new::command())
}

fn main() {
    let matches = root().get_matches();

    match matches.subcommand() {
        | Some(("doctor", args)) => doctor::execute(args),
        | Some(("new", args)) => new::execute(args),
        | Some((_, _)) => panic!("Not implemented"),
        | None => panic!("No command provided"),
    }
}
