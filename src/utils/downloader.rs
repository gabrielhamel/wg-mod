use std::fs::File;
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum DownloadError {
    #[error("Failed to GET")]
    FetchError(#[from] reqwest::Error),

    #[error("Error occurred during file writing")]
    FileWriteError(#[from] io::Error),
}

pub fn download_file(url: &str, path: &str) -> Result<(), DownloadError> {
    let mut http_response = reqwest::blocking::get(url)?;
    let mut file = File::create(path)?;

    io::copy(&mut http_response, &mut file)?;
    Ok(())
}
