use crate::cli::command::{CommandError, RunnableCommand};
use crate::config::{Configs, ConfigsError};
use crate::sdk::game_sources::GameSourcesError;
use clap::{ArgMatches, Command};

#[derive(thiserror::Error, Debug)]
pub enum ChannelCommandError {
    #[error("Failed to load modding tools\n{0}")]
    ConfigsError(#[from] ConfigsError),

    #[error("Failed to use build tools\n{0}")]
    GameSourceError(#[from] GameSourcesError),
}

pub struct ChannelCommand;

fn channel() -> Result<(), ChannelCommandError> {
    let config = Configs::load()?;
    let channel = config.game_sources.get_channel()?;

    println!("Current WoT channel: {}", channel);
    Ok(())
}

fn switch_channel() -> Result<(), ChannelCommandError> {
    let config = Configs::load()?;
    config.game_sources.prompt_channel()?;

    Ok(())
}

impl RunnableCommand for ChannelCommand {
    fn command() -> Command {
        Command::new("channel")
            .about("Display / set current WoT region selected")
            .long_about("Display / set current WoT region selected")
            .subcommand_required(false)
            .subcommand(
                Command::new("switch")
                    .about("Change WoT region")
                    .long_about("Change WoT region currently selected"),
            )
    }

    fn run(args: &ArgMatches) -> Result<(), CommandError> {
        if let Some(_) = args.subcommand_matches("switch") {
            return match switch_channel() {
                | Ok(()) => Ok(()),
                | Err(e) => {
                    Err(CommandError::CommandExecutionError(e.to_string()))
                },
            };
        }

        match channel() {
            | Ok(()) => Ok(()),
            | Err(e) => Err(CommandError::CommandExecutionError(e.to_string())),
        }
    }
}
