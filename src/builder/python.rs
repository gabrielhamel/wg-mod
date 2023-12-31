use crate::config::{Configs, ConfigsError};
use crate::sdk::conda::environment::{CondaEnvironment, CondaEnvironmentError};
use crate::sdk::conda::CondaError;
use crate::utils::copy_directory::{copy_directory, CopyDirectoryError};
use glob::glob;
use std::fs::{create_dir_all, remove_file};
use std::io;
use std::path::PathBuf;
use tempfile::tempdir;

#[derive(thiserror::Error, Debug)]
pub enum PythonBuilderError {
    #[error("Conda environment error\n{0}")]
    CondaEnvironmentError(#[from] CondaEnvironmentError),

    #[error("Conda error\n{0}")]
    CondaError(#[from] CondaError),

    #[error("Can't access to configs")]
    ConfigsError(#[from] ConfigsError),

    #[error("Unable to create a directory")]
    CreateDirectoryError(#[from] io::Error),

    #[error("Copy directory failed")]
    CopyDirectoryError(#[from] CopyDirectoryError),

    #[error("Invalid path given")]
    PathError,

    #[error("File selection by pattern failed")]
    GlobError(#[from] glob::GlobError),

    #[error("Invalid pattern given")]
    PatternError(#[from] glob::PatternError),

    #[error("Can't copy or create files\n{0}")]
    WriteFilesError(io::Error),
}

pub struct PythonBuilder {
    conda_environment: CondaEnvironment,
}

impl PythonBuilder {
    pub fn new() -> Result<Self, PythonBuilderError> {
        let configs = Configs::load()?;

        Ok(Self {
            conda_environment: configs.conda_environment,
        })
    }

    pub fn build(
        &self, source: &PathBuf, destination: &PathBuf,
    ) -> Result<(), PythonBuilderError> {
        let (close_tmp_dir, tmp_dir_path) = self.prepare_tmp_directory()?;

        copy_directory(source, &tmp_dir_path)?;

        self.conda_environment.compile_all(&tmp_dir_path)?;
        self.delete_all_sources(&tmp_dir_path)?;

        create_dir_all(destination)?;
        copy_directory(&tmp_dir_path, destination)?;

        close_tmp_dir()?;

        Ok(())
    }

    fn prepare_tmp_directory(
        &self,
    ) -> Result<(impl FnOnce() -> io::Result<()>, PathBuf), PythonBuilderError>
    {
        let tmp_dir = tempdir()?;
        let path = tmp_dir.path();
        let path_buf = path.to_path_buf();

        let close_tmp_dir = move || tmp_dir.close();

        Ok((close_tmp_dir, path_buf))
    }

    fn delete_all_sources(
        &self, directory: &PathBuf,
    ) -> Result<(), PythonBuilderError> {
        let directory_path =
            directory.to_str().ok_or(PythonBuilderError::PathError)?;
        let glob_pattern = format!("{}/**/*.py", directory_path);

        let remaining_python_files = glob(&glob_pattern)?;

        for entry in remaining_python_files {
            let file = entry?;
            remove_file(file).map_err(PythonBuilderError::WriteFilesError)?;
        }

        Ok(())
    }
}
