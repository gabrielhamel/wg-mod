use fs_extra::dir::{get_dir_content, CopyOptions};
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum CopyDirectoryError {
    #[error("Cannot copy directory {0} to {1}\n{2}")]
    CopyDirectoryError(PathBuf, PathBuf, fs_extra::error::Error),

    #[error("Cannot read the directory")]
    GetDirectoryContentError(#[from] fs_extra::error::Error),
}

pub fn copy_directory(
    source: &PathBuf, destination: &PathBuf,
) -> Result<(), CopyDirectoryError> {
    let options = CopyOptions::new();
    let content = get_dir_content(source)?;

    fs_extra::copy_items(&content.files, destination.as_path(), &options)
        .map_err(|e| {
            CopyDirectoryError::CopyDirectoryError(
                source.clone(),
                destination.clone(),
                e,
            )
        })?;

    Ok(())
}
