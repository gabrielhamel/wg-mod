mod python;

use crate::builder::python::{PythonBuilder, PythonBuilderError};
use fs_extra::dir;
use glob::glob;
use std::path::PathBuf;
use std::{fs, io};
use zip::result::ZipError;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip_extensions::*;

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

    #[error("Can't copy or create files\n{0}")]
    WriteFilesError(io::Error),

    #[error("Unable to write mod archive\n{0}")]
    ZipWriteError(#[from] ZipError),
}

pub struct ModBuilder {
    python_builder: PythonBuilder,
    mod_path: PathBuf,
    target_directory: PathBuf,
    build_path: PathBuf,
}

impl ModBuilder {
    pub fn new(mod_path: PathBuf) -> Result<Self, ModBuilderError> {
        let python_builder = PythonBuilder::new()?;
        let target_directory = mod_path.join("target");
        let build_path = target_directory.join("release").join("build");

        Ok(Self {
            python_builder,
            mod_path,
            target_directory,
            build_path,
        })
    }

    fn clean_target_directory(&self) -> Result<(), ModBuilderError> {
        let _ = fs::remove_dir_all(&self.target_directory);

        fs::create_dir_all(&self.build_path)
            .map_err(ModBuilderError::WriteFolderError)?;

        Ok(())
    }

    fn build_python_src(&self) -> Result<(), ModBuilderError> {
        let python_src_path = self.mod_path.join("scripts");
        let python_build_destination =
            self.build_path.join("res/scripts/client/gui");

        fs::create_dir_all(&python_build_destination)
            .map_err(ModBuilderError::WriteFolderError)?;

        let options = dir::CopyOptions::new();
        fs_extra::copy_items(
            &[python_src_path.as_path()],
            python_build_destination.as_path(),
            &options,
        )?;

        fs::rename(
            python_build_destination.join("scripts"),
            python_build_destination.join("mods"),
        )
        .map_err(ModBuilderError::WriteFolderError)?;

        let target_python_path = python_build_destination.join("mods");
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

    fn copy_meta_file(&self) -> Result<(), ModBuilderError> {
        let meta_path = self.mod_path.join("meta.xml");
        let build_directory = self.build_path.join("meta.xml");
        fs::copy(meta_path, build_directory)
            .map_err(ModBuilderError::WriteFilesError)?;

        Ok(())
    }

    fn make_archive(&self) -> Result<(), ModBuilderError> {
        let archive_file =
            self.target_directory.join("release").join("result.wotmod");

        zip_create_from_directory_with_options(
            &archive_file,
            &self.build_path,
            FileOptions::default()
                .compression_method(CompressionMethod::Stored),
        )?;

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
        self.copy_meta_file()?;
        self.make_archive()?;

        let canonicalize_path = self
            .target_directory
            .join("release")
            .join("result.wotmod")
            .canonicalize()
            .map_err(|_| ModBuilderError::PathError)?;

        let str_path = canonicalize_path
            .to_str()
            .ok_or(ModBuilderError::PathError)?
            .replace("\\\\?\\", "");

        println!("Build finished: {}", str_path);

        Ok(())
    }

    fn is_mod_folder(&self) -> bool {
        let meta_path = self.mod_path.join("meta.xml");
        meta_path.exists()
    }
}
