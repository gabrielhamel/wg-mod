use crate::config;
use crate::config::Configs;
use crate::sdk::asconfigc;
use crate::sdk::asconfigc::ASConfigc;
use crate::utils::copy_directory;
use crate::utils::tmp_dir::TempDirError;
use glob::{GlobError, PatternError};
use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to get asconfigc: {0}")]
    AsConfigcError(#[from] config::Error),
    #[error("Failed to manage temporary directory: {0}")]
    TempDirError(#[from] TempDirError),
    #[error("Failed to write file and directory: {0}")]
    WriteError(#[from] io::Error),
    #[error("Failed to build: {0}")]
    BuildError(#[from] asconfigc::Error),
    #[error("Failed to copy directory: {0}")]
    CopyError(#[from] copy_directory::Error),
    #[error("Failed to unwrap glob: {0}")]
    GlobUnwrapError(#[from] GlobError),
    #[error("Failed to creta glob: {0}")]
    GlobCreationError(#[from] PatternError),
}

pub struct FlashBuilder {
    asconfigc: ASConfigc,
}

impl FlashBuilder {
    pub fn new() -> Result<Self, Error> {
        let config = Configs::load().map_err(|e| Error::AsConfigcError(e))?;

        Ok(Self {
            asconfigc: config.asconfigc,
        })
    }

    pub fn build(
        &self, source: &PathBuf, destination: &PathBuf,
    ) -> Result<(), Error> {
        self.asconfigc.build(source)?;

        create_dir_all(destination)?;

        Ok(())
    }
}
