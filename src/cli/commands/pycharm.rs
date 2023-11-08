use crate::cli::command::{CommandError, RunnableCommand};
use crate::config::{Configs, ConfigsError};
use crate::sdk::game_sources::GameSourcesError;
use clap::{ArgMatches, Command};

#[derive(thiserror::Error, Debug)]
pub enum PycharmCommandError {
    #[error("Failed to load modding tools\n{0}")]
    ConfigsError(#[from] ConfigsError),

    #[error("Failed to use build tools\n{0}")]
    GameSourceError(#[from] GameSourcesError),
}

pub struct PycharmCommand;

fn pycharm() -> Result<(), PycharmCommandError> {
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

    fn run(_: &ArgMatches) -> Result<(), CommandError> {
        match pycharm() {
            | Ok(()) => Ok(()),
            | Err(e) => Err(CommandError::CommandExecutionError(e.to_string())),
        }
    }
}
