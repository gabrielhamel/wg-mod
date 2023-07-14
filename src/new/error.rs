use std::path::PathBuf;
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum NewError {
    #[error("The provided path is invalid")]
    PathError,

    #[error("The provided path {0} not exists")]
    PathNotExists(PathBuf),

    #[error("Unable to create this directory {0}")]
    UnableToCreateDirectory(io::Error),
}
