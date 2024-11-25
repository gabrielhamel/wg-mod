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

pub struct ChannelCommand;

fn channel() -> Result<()> {
    let config = Configs::load()?;
    let channel = config.game_sources.get_channel()?;

    println!("Current WoT channel: {}", channel);
    Ok(())
}

fn switch_channel() -> Result<()> {
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

    fn run(args: &ArgMatches) -> result::Result<(), command::Error> {
        if let Some(_) = args.subcommand_matches("switch") {
            return match switch_channel() {
                | Ok(()) => Ok(()),
                | Err(e) => {
                    Err(command::Error::CommandExecutionError(e.to_string()))
                },
            };
        }

        match channel() {
            | Ok(()) => Ok(()),
            | Err(e) => {
                Err(command::Error::CommandExecutionError(e.to_string()))
            },
        }
    }
}
