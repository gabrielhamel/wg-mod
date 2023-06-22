use clap::Command;

mod doctor;

fn root() -> Command {
    Command::new("wg-mod")
        .version("0.1.0")
        .author("Gabriel Hamel <gabriel.hamel.pro@gmail.com>")
        .about("Provides tools for wargaming modding")
        .subcommand_required(true)
        .subcommand(doctor::command())
}

fn main() {
    let matches = root().get_matches();

    match matches.subcommand() {
        | Some(("doctor", args)) => doctor::execute(args),
        | Some((_, _)) => panic!("Not implemented"),
        | None => panic!("No command provided"),
    }
}
