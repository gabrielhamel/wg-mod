mod python;

use crate::builder::python::{PythonBuilder, PythonBuilderError};
use fs_extra::dir;
use glob::glob;
use std::path::PathBuf;
use std::{fs, io};

#[derive(thiserror::Error, Debug)]
pub enum ModBuilderError {
    #[error("Failed to use the python mod builder\n{0}")]
    PythonBuilderError(#[from] PythonBuilderError),

    #[error("Copy directory failed\n{0}")]
    CopyDirectoryError(#[from] fs_extra::error::Error),

    #[error("Glob error\n{0}")]
    GlobError(#[from] glob::GlobError),

    #[error("The path \"{0}\" isn't a mod folder")]
    BadModFolderError(PathBuf),

    #[error("Path error")]
    PathError,

    #[error("Can't create target directory")]
    WriteFolderError(io::Error),

    #[error("Can't copy or create files")]
    WriteFilesError(io::Error),
}

pub struct ModBuilder {
    python_builder: PythonBuilder,
    mod_path: PathBuf,
    target_directory: PathBuf,
}

impl ModBuilder {
    pub fn new(mod_path: PathBuf) -> Result<Self, ModBuilderError> {
        let python_builder = PythonBuilder::new()?;
        let target_directory = mod_path.join("target");

        Ok(Self {
            python_builder,
            mod_path,
            target_directory,
        })
    }

    fn clean_target_directory(&self) -> Result<(), ModBuilderError> {
        let _ = fs::remove_dir_all(&self.target_directory);

        fs::create_dir_all(&self.target_directory)
            .map_err(ModBuilderError::WriteFolderError)?;

        Ok(())
    }

    fn build_python_src(&self) -> Result<(), ModBuilderError> {
        let python_src_path = self.mod_path.join("scripts");

        let options = dir::CopyOptions::new();
        fs_extra::copy_items(
            &[python_src_path.as_path()],
            self.target_directory.as_path(),
            &options,
        )?;

        let target_python_path = self.target_directory.join("scripts");
        self.python_builder.build(&target_python_path)?;

        let target_python_path_str = target_python_path
            .to_str()
            .ok_or(ModBuilderError::PathError)?;
        let glob_pattern = format!("{}/**/*.py", target_python_path_str);

        let remaining_python_files =
            glob(&glob_pattern).map_err(|_| ModBuilderError::PathError)?;
        for entry in remaining_python_files {
            let file = entry?;
            fs::remove_file(file).map_err(ModBuilderError::WriteFilesError)?;
        }

        Ok(())
    }

    pub fn build(&self) -> Result<(), ModBuilderError> {
        let is_mod_folder = self.is_mod_folder();
        if is_mod_folder == false {
            let absolute_mod_folder_path = fs::canonicalize(&self.mod_path)
                .map_err(|_| ModBuilderError::PathError)?;

            return Err(ModBuilderError::BadModFolderError(
                absolute_mod_folder_path,
            ));
        }

        self.clean_target_directory()?;
        self.build_python_src()?;

        Ok(())
    }

    fn is_mod_folder(&self) -> bool {
        let meta_path = self.mod_path.join("meta.xml");
        meta_path.exists()
    }
}
