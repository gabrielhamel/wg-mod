use fs_extra::dir::{get_dir_content, CopyOptions};
use std::path::PathBuf;
use std::result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot copy directory {0} to {1}\n{2}")]
    CopyDirectoryError(PathBuf, PathBuf, fs_extra::error::Error),

    #[error("Cannot read the directory")]
    GetDirectoryContentError(#[from] fs_extra::error::Error),
}

type Result<T> = result::Result<T, Error>;

pub fn copy_directory(source: &PathBuf, destination: &PathBuf) -> Result<()> {
    let options = CopyOptions::new();
    let content = get_dir_content(source)?;

    fs_extra::copy_items(&content.files, destination.as_path(), &options)
        .map_err(|e| {
            Error::CopyDirectoryError(source.clone(), destination.clone(), e)
        })?;

    Ok(())
}
