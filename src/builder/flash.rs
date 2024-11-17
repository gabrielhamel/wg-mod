use crate::builder::python::PythonBuilderError;
use crate::config::Configs;
use crate::sdk::asconfigc::ASConfigc;
use crate::utils::copy_directory::copy_directory;
use crate::utils::tmp_dir::prepare_tmp_directory;
use std::fs::{create_dir_all, remove_file};
use std::path::PathBuf;
use glob::glob;

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

        println!("{}", tmp_dir_path.to_str().unwrap());

        self.asconfigc
            .build(&tmp_dir_path)
            .map_err(|_| FlashBuilderError::BuildError)?;
        self.delete_all_sources(&tmp_dir_path)?;

        create_dir_all(destination)
            .map_err(|_| FlashBuilderError::BuildError)?;
        copy_directory(&tmp_dir_path, destination)
            .map_err(|_| FlashBuilderError::BuildError)?;

        close_tmp_dir().map_err(|_| FlashBuilderError::BuildError)?;

        Ok(())
    }

    // TODO -> Exact Same code as Python_builder.delete_all_source(), just file extention change
    // TODO -> To rebase
    fn delete_all_sources(
        &self, directory: &PathBuf,
    ) -> Result<(), FlashBuilderError> {
        let directory_path =
            directory.to_str().ok_or(PythonBuilderError::PathError)?;
        let glob_pattern = format!("{}/**/*.as", directory_path);

        let remaining_python_files = glob(&glob_pattern)?;

        for entry in remaining_python_files {
            let file = entry?;
            remove_file(file).map_err(PythonBuilderError::WriteFilesError)?;
        }

        Ok(())
    }
}
