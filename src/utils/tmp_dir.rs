use std::io;
use std::path::PathBuf;
use tempfile::tempdir;

#[derive(Debug, thiserror::Error)]
pub enum TempDirError {
    #[error("Failed to create temporary directory")]
    TempDirError(#[from] io::Error),
}

pub fn prepare_tmp_directory(
) -> Result<(impl FnOnce() -> io::Result<()>, PathBuf), TempDirError> {
    let tmp_dir = tempdir()?;
    let path = tmp_dir.path();
    let path_buf = path.to_path_buf();

    let close_tmp_dir = move || tmp_dir.close();

    Ok((close_tmp_dir, path_buf))
}
