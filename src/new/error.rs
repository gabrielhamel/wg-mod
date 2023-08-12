use std::io;
use std::path::PathBuf;

use crate::utils::file_template;

#[derive(thiserror::Error, Debug)]
pub enum NewError {
    #[error("The provided path is invalid")]
    PathError,

    #[error("The provided path {0} not exists")]
    PathNotExists(PathBuf),

    #[error("Unable to create this directory {0}")]
    UnableToCreateDirectory(io::Error),

    #[error("Cannot build the regex")]
    InvalidRegex(#[from] regex::Error),

    #[error("Cannot parse arguments")]
    InvalidPromptRegex(#[from] inquire::InquireError),

    #[error("Cannot create templated file")]
    FileTemplateError(#[from] file_template::FileTemplateError),
}
