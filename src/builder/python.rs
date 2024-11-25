use crate::config;
use crate::config::Configs;
use crate::sdk::conda;
use crate::sdk::conda::environment::CondaEnvironment;
use crate::utils::copy_directory;
use crate::utils::copy_directory::copy_directory;
use glob::glob;
use std::fs::{create_dir_all, remove_file};
use std::path::PathBuf;
use std::{io, result};
use tempfile::tempdir;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Conda environment error\n{0}")]
    CondaEnvironmentError(#[from] conda::environment::Error),

    #[error("Conda error\n{0}")]
    CondaError(#[from] conda::Error),

    #[error("Can't access to configs")]
    ConfigsError(#[from] config::Error),

    #[error("Unable to create a directory")]
    CreateDirectoryError(#[from] io::Error),

    #[error("Copy directory failed")]
    CopyDirectoryError(#[from] copy_directory::Error),

    #[error("Invalid path given")]
    PathError,

    #[error("File selection by pattern failed")]
    GlobError(#[from] glob::GlobError),

    #[error("Invalid pattern given")]
    PatternError(#[from] glob::PatternError),

    #[error("Can't copy or create files\n{0}")]
    WriteFilesError(io::Error),
}

type Result<T> = result::Result<T, Error>;

pub struct PythonBuilder {
    conda_environment: CondaEnvironment,
}

impl PythonBuilder {
    pub fn new() -> Result<Self> {
        let configs = Configs::load()?;

        Ok(Self {
            conda_environment: configs.conda_environment,
        })
    }

    pub fn build(&self, source: &PathBuf, destination: &PathBuf) -> Result<()> {
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
    ) -> Result<(impl FnOnce() -> io::Result<()>, PathBuf)> {
        let tmp_dir = tempdir()?;
        let path = tmp_dir.path();
        let path_buf = path.to_path_buf();

        let close_tmp_dir = move || tmp_dir.close();

        Ok((close_tmp_dir, path_buf))
    }

    fn delete_all_sources(&self, directory: &PathBuf) -> Result<()> {
        let directory_path = directory.to_str().ok_or(Error::PathError)?;
        let glob_pattern = format!("{}/**/*.py", directory_path);

        let remaining_python_files = glob(&glob_pattern)?;

        for entry in remaining_python_files {
            let file = entry?;
            remove_file(file).map_err(Error::WriteFilesError)?;
        }

        Ok(())
    }
}
