use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ConvertAbsolutePathError {
    #[error("Cannot get absolute path of {0}")]
    ConvertAbsolutePathError(PathBuf),
}

pub fn convert_to_absolute_path(
    path: &PathBuf,
) -> Result<String, ConvertAbsolutePathError> {
    let absolute_path = path.canonicalize().map_err(|e| {
        eprintln!("{:?}", e);
        ConvertAbsolutePathError::ConvertAbsolutePathError(path.clone())
    })?;

    let str_path = absolute_path.to_str().ok_or(
        ConvertAbsolutePathError::ConvertAbsolutePathError(path.clone()),
    )?;

    if cfg!(target_os = "windows") {
        let path_without_prefix = str_path.replace("\\\\?\\", "");
        Ok(path_without_prefix)
    } else {
        Ok(str_path.to_string())
    }
}
