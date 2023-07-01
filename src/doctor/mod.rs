mod error;

use clap::{ArgMatches, Command};

use self::error::DoctorError;
use super::config::Configs;

pub fn command() -> Command {
    Command::new("doctor").about("Check the sanity of the installation")
}

pub fn doctor() -> Result<(), DoctorError> {
    let configs = Configs::load()?;

    println!("tool home: {}", configs.wg_mod_home);
    Ok(())
}

pub fn execute(_: &ArgMatches) {
    println!("Doctor summary:");

    match doctor() {
        | Ok(()) => println!("• No issues found!"),
        | Err(e) => eprintln!("Error {}", e),
    }
}
