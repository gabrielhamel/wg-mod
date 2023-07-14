mod error;

use self::error::NewError;
use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;
use std::fs::create_dir;

struct NewModArguments {
    name: String,
    path: PathBuf,
}

pub fn command() -> Command {
    Command::new("new")
        .about("Create a new mod project")
        .long_about("Create a new mod project at <path>")
        .arg(Arg::new("path").required(true))
        .arg(
            Arg::new("NAME")
                .long("name")
                .help("Set the resulting mod name, defaults to the directory name"),
        )
}

fn parse_new_mod_args(args: &ArgMatches) -> Result<NewModArguments, NewError> {
    let path = args.get_one::<String>("path").ok_or(NewError::PathError)?;
    let name = args.get_one::<String>("NAME");

    let mut complete_path = PathBuf::from(path);
    if let Some(name) = name {
        if !complete_path.exists() {
            return Err(NewError::PathNotExists(complete_path));
        }
        complete_path.push(name);
    }

    let mod_name = String::from(
        complete_path
            .file_name()
            .and_then(|filename| filename.to_str())
            .ok_or(NewError::PathError)?,
    );


    Ok(NewModArguments {
        name: mod_name,
        path: complete_path
    })
}

pub fn new(args: &ArgMatches) -> Result<(), NewError> {
    let args = parse_new_mod_args(args)?;

    // // Create mod directory
    // create_dir(args.path).map_err(NewError::UnableToCreateDirectory)?;


    Ok(())
}

pub fn execute(args: &ArgMatches) {
    if let Err(e) = new(args) {
        eprintln!("Error {}", e)
    }
}
