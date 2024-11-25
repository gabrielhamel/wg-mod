use crate::cli::command;
use crate::cli::command::RunnableCommand;
use crate::config;
use crate::config::Configs;
use crate::sdk::game_sources;
use clap::{ArgMatches, Command};
use std::result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to load modding tools\n{0}")]
    ConfigsError(#[from] config::Error),

    #[error("Failed to use build tools\n{0}")]
    GameSourceError(#[from] game_sources::Error),
}

type Result<T> = result::Result<T, Error>;

pub struct PycharmCommand;

fn pycharm() -> Result<()> {
    let config = Configs::load()?;
    let python_root_modules = config.game_sources.list_python_root_modules()?;

    println!("Resolve WoT imports:
1. Go in your PyCharm project settings
2. Open Python Interpreter tab
3. Click on your actual conda interpreter (must be wg-mod) and on the button 'Show all'
4. Select 'wg-mod' and click on the small directories icon 'Show Interpreter Paths'
5. Add these following paths by clicking the button '+'
");
    for module in python_root_modules {
        println!(" - {module}");
    }

    Ok(())
}

impl RunnableCommand for PycharmCommand {
    fn command() -> Command {
        Command::new("pycharm")
            .about("Help to configure your local PyCharm IDE")
    }

    fn run(_: &ArgMatches) -> result::Result<(), command::Error> {
        match pycharm() {
            | Ok(()) => Ok(()),
            | Err(e) => {
                Err(command::Error::CommandExecutionError(e.to_string()))
            },
        }
    }
}
