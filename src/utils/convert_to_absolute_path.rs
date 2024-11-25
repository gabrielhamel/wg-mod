use std::path::PathBuf;
use std::result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot get absolute path of {0}")]
    ConvertAbsolutePathError(PathBuf),
}

type Result<T> = result::Result<T, Error>;

pub fn convert_to_absolute_path(path: &PathBuf) -> Result<String> {
    let absolute_path = path.canonicalize().map_err(|e| {
        eprintln!("{:?}", e);
        Error::ConvertAbsolutePathError(path.clone())
    })?;

    let str_path = absolute_path
        .to_str()
        .ok_or(Error::ConvertAbsolutePathError(path.clone()))?;

    if cfg!(target_os = "windows") {
        let path_without_prefix = str_path.replace("\\\\?\\", "");
        Ok(path_without_prefix)
    } else {
        Ok(str_path.to_string())
    }
}
