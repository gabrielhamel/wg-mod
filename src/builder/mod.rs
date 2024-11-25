mod flash;
mod python;

use crate::builder::flash::{FlashBuilder, FlashBuilderError};
use crate::builder::python::PythonBuilder;
use crate::utils::convert_to_absolute_path;
use crate::utils::convert_to_absolute_path::convert_to_absolute_path;
use std::path::PathBuf;
use std::{fs, io, result};
use zip::result::ZipError;
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip_extensions::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to use the python mod builder\n{0}")]
    PythonBuilderError(#[from] python::Error),

    #[error("Failed to use the flash mod builder\n{0}")]
    FlashBuilderError(#[from] FlashBuilderError),

    #[error("Copy directory failed\n{0}")]
    CopyDirectoryError(#[from] fs_extra::error::Error),

    #[error("Glob error\n{0}")]
    GlobError(#[from] glob::GlobError),

    #[error("The path \"{0}\" isn't a mod folder")]
    BadModFolderError(PathBuf),

    #[error("Path error")]
    PathError,

    #[error("Can't copy or create files\n{0}")]
    WriteFilesError(io::Error),

    #[error("Unable to write mod archive\n{0}")]
    ZipWriteError(#[from] ZipError),

    #[error("Unable to get the absolute path of the archive")]
    ConvertAbsolutePathError(#[from] convert_to_absolute_path::Error),
}

type Result<T> = result::Result<T, Error>;

pub struct ModBuilder {
    python_builder: PythonBuilder,
    flash_builder: FlashBuilder,
    mod_path: PathBuf,
    target_path: PathBuf,
    build_path: PathBuf,
}

impl ModBuilder {
    pub fn new(mod_path: PathBuf) -> Result<Self> {
        let python_builder = PythonBuilder::new()?;
        let flash_builder = FlashBuilder::new()?;
        let target_path = mod_path.join("target");
        let build_path = target_path.join("build");

        Ok(Self {
            python_builder,
            flash_builder,
            mod_path,
            target_path,
            build_path,
        })
    }

    fn clean_target_directory(&self) -> Result<()> {
        let _ = fs::remove_dir_all(&self.target_path);

        Ok(())
    }

    fn build_python_src(&self) -> Result<()> {
        let python_sources = self.mod_path.join("scripts");
        let python_build_destination =
            self.build_path.join("res/scripts/client/gui/mods");

        self.python_builder
            .build(&python_sources, &python_build_destination)?;

        Ok(())
    }

    fn build_flash_src(&self) -> Result<()> {
        let flash_sources = self.mod_path.join("ui");
        let flash_build_destination = self.build_path.join("res/gui/flash");

        self.flash_builder
            .build(&flash_sources, &flash_build_destination)?;

        Ok(())
    }

    fn copy_meta_file(&self) -> Result<()> {
        let meta_path = self.mod_path.join("meta.xml");
        let build_directory = self.build_path.join("meta.xml");
        fs::copy(meta_path, build_directory).map_err(Error::WriteFilesError)?;

        Ok(())
    }

    fn make_archive(&self) -> Result<PathBuf> {
        let archive_file = self.target_path.join("result.wotmod");
        let compression_options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored);

        zip_create_from_directory_with_options(
            &archive_file,
            &self.build_path,
            |_| compression_options,
        )?;

        Ok(archive_file)
    }

    pub fn build(&self) -> Result<()> {
        self.throw_if_isn_t_mod_folder()?;

        self.clean_target_directory()?;

        self.build_python_src()?;
        self.copy_meta_file()?;

        self.build_flash_src()?;

        let archive_path = self.make_archive()?;
        let absolute_build_path = convert_to_absolute_path(&archive_path)?;
        println!("Build finished: {}", absolute_build_path);

        Ok(())
    }

    fn throw_if_isn_t_mod_folder(&self) -> Result<()> {
        let meta_path = self.mod_path.join("meta.xml");

        if meta_path.exists() == false {
            let absolute_mod_folder_path =
                self.mod_path.canonicalize().map_err(|_| Error::PathError)?;

            return Err(Error::BadModFolderError(absolute_mod_folder_path));
        };

        Ok(())
    }
}
