use crate::config;
use crate::utils::convert_pathbuf_to_string::Stringify;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("File interaction failed : {0}")]
    FileError(#[from] std::io::Error),
    #[error("Failed to read json : {0}")]
    ParsingError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(skip)]
    pub settings_file_path: PathBuf,
    #[serde(rename = "game_client_path")]
    pub game_client_path: Option<PathBuf>,
}

impl Settings {
    pub fn create_default_settings(settings_file_path: PathBuf) -> Self {
        Self {
            settings_file_path,
            game_client_path: None,
        }
    }

    fn prompt_game_client_path(&self) -> Result<String, config::Error> {
        let default_game_client_path = if cfg!(target_os = "windows") {
            PathBuf::from("C:\\Games\\World_of_Tanks_EU")
        } else {
            let user = env::var("USER")?;
            PathBuf::from(format!(
                "/Users/{user}/Documents/Wargaming.net Games/World_of_Tanks_EU"
            ))
        };
        let default_game_client_str = default_game_client_path.to_string()?;

        let value = inquire::Text::new("WoT client path:")
            .with_default(default_game_client_str.as_str())
            .prompt()
            .map_err(config::Error::PromptError)?;

        Ok(value)
    }

    pub fn verify_game_client_path_validity(&mut self) {
        let is_path_valid =
            if let Some(game_client_path) = &self.game_client_path {
                game_client_path.exists()
            } else {
                false
            };

        if !is_path_valid {
            let new_path = self.prompt_game_client_path();
            let is_path_valid = match &new_path {
                | Ok(path) => {
                    let path_buf = PathBuf::from(path);
                    if path_buf.exists() {
                        self.game_client_path = Some(path_buf);
                        true
                    } else {
                        self.game_client_path = None;
                        false
                    }
                },
                | Err(_) => {
                    self.game_client_path = None;
                    false
                },
            };

            if !is_path_valid {
                println!();
                println!("--- Pay attention ---");
                println!("Game client path doesn't exist: {:?}", new_path);
                println!("You will not be able to build");
                println!("to set it, rerun wg-mod command");
                println!("---------------------");
                println!();
            }
        }
    }

    pub fn write_to_json_file(&self) -> Result<(), Error> {
        let file = File::create(&self.settings_file_path)?;

        serde_json::to_writer_pretty(file, self)
            .map_err(|e| Error::ParsingError(e.to_string()))?;

        Ok(())
    }

    pub fn from_json_file(filename: &PathBuf) -> Result<Settings, Error> {
        let file = File::open(filename)?;
        let mut settings: Settings = serde_json::from_reader(file)
            .map_err(|e| Error::ParsingError(e.to_string()))?;
        settings.settings_file_path = filename.clone();
        Ok(settings)
    }
}
