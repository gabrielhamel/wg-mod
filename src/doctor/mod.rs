use clap::{ArgMatches, Command};

pub fn command() -> Command {
    Command::new("doctor").about("Check the sanity of the installation")
}

pub fn execute(args: &ArgMatches) {
    println!("Execute doctor command")
}
