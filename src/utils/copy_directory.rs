use fs_extra::dir::CopyOptions;
use std::fs::read_dir;
use std::path::PathBuf;
use std::{io, result};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot copy directory {0} to {1}\n{2}")]
    CopyDirectoryError(PathBuf, PathBuf, fs_extra::error::Error),

    #[error("Cannot read the directory")]
    GetDirectoryContentError(#[from] io::Error),
}

type Result<T> = result::Result<T, Error>;

pub fn copy_directory(source: &PathBuf, destination: &PathBuf) -> Result<()> {
    let options = CopyOptions::new();
    let content = read_dir(source)?
        .flatten()
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    fs_extra::copy_items(&content, destination.as_path(), &options).map_err(
        |e| Error::CopyDirectoryError(source.clone(), destination.clone(), e),
    )?;

    Ok(())
}
