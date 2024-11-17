use crate::builder::python::PythonBuilderError;
use crate::config::Configs;
use crate::sdk::asconfigc::ASConfigc;
use crate::utils::copy_directory::copy_directory;
use crate::utils::tmp_dir::prepare_tmp_directory;
use std::fs::create_dir_all;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum FlashBuilderError {
    #[error("Failed to get asconfigc")]
    AsConfigcError,
    #[error("Failed to build")]
    BuildError,
}

pub struct FlashBuilder {
    asconfigc: ASConfigc,
}

impl FlashBuilder {
    pub fn new() -> Result<Self, FlashBuilderError> {
        let config =
            Configs::load().map_err(|_| FlashBuilderError::AsConfigcError)?;

        Ok(Self {
            asconfigc: config.asconfigc,
        })
    }

    pub fn build(
        &self, source: &PathBuf, destination: &PathBuf,
    ) -> Result<(), FlashBuilderError> {
        let (close_tmp_dir, tmp_dir_path) = prepare_tmp_directory()
            .map_err(|_| FlashBuilderError::BuildError)?;
        copy_directory(source, &tmp_dir_path)
            .map_err(|_| FlashBuilderError::BuildError)?;

        self.asconfigc
            .compile_all(&tmp_dir_path)
            .map_err(|_| FlashBuilderError::BuildError)?;
        // self.delete_all_sources(&tmp_dir_path)?;

        create_dir_all(destination)
            .map_err(|_| FlashBuilderError::BuildError)?;
        copy_directory(&tmp_dir_path, destination)
            .map_err(|_| FlashBuilderError::BuildError)?;

        close_tmp_dir().map_err(|_| FlashBuilderError::BuildError)?;

        Ok(())
    }
}
