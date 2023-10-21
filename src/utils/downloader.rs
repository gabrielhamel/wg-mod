use std::fs::File;
use std::io::Write;

#[derive(thiserror::Error, Debug)]
pub enum DownloadError {
    #[error("Failed to GET")]
    FetchError(#[from] reqwest::Error),

    #[error("Error occurred during file writing")]
    FileWriteError(#[from] std::io::Error),
}

pub fn download_file(url: &str, path: &str) -> Result<(), DownloadError> {
    let http_response = reqwest::blocking::get(url)?;
    let bytes = http_response.bytes()?;

    let mut file = File::create(path)?;
    file.write_all(&bytes)?;

    Ok(())
}
