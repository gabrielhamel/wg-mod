use std::fs::File;
use std::{io, result};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An error occurred while downloading the file")]
    Fetch(#[from] reqwest::Error),

    #[error("An error occurred while saving the file")]
    Io(#[from] io::Error),
}

type Result<T> = result::Result<T, Error>;

pub fn download_file(url: &str, path: &str) -> Result<()> {
    let mut http_response = reqwest::blocking::get(url)?;
    let mut file = File::create(path)?;

    io::copy(&mut http_response, &mut file)?;
    Ok(())
}
