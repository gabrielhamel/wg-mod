use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed cast: {0}")]
    CastError(String),
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
    pub game_client_path: PathBuf,
}

impl Settings {
    pub fn create_default_settings(settings_file_path: PathBuf) -> Self {
        Self {
            settings_file_path,
            game_client_path: PathBuf::from(""),
        }
    }

    pub fn is_game_client_path_set(&self) -> bool {
        self.game_client_path.exists()
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
