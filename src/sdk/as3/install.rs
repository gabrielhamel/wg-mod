use crate::utils::{convert_to_absolute_path, downloader};
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io, result};
use zip::result::ZipError;
use zip::ZipArchive;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No ActionScript 3 SDK is available for this platform")]
    PlatformNotSupported,

    #[error("Unable to resolve download destination")]
    ConvertAbsolutePath(#[from] convert_to_absolute_path::Error),

    #[error("An error occurred during the SDK download")]
    SdkDownloadFailed(#[from] downloader::Error),

    #[error("Fail to resolve the following path: {0}")]
    ResolvePathFailed(PathBuf),

    #[error("Invalid file downloaded")]
    InvalidArchive(io::Error),

    #[error("Invalid Zip operation")]
    InvalidZipOperation(#[from] ZipError),

    #[error("Unable to delete archive")]
    DeleteArchiveFailed(io::Error),
}

type Result<T> = result::Result<T, Error>;

fn get_archive_url() -> Result<String> {
    let os = match std::env::consts::OS {
        | "macos" => Ok("macos"),
        | "windows" => Ok("windows"),
        | _ => Err(Error::PlatformNotSupported),
    }?;

    Ok(format!(
        "https://wg-mod.s3.eu-west-3.amazonaws.com/apache-flex-{os}.zip"
    ))
}

fn get_sdk_archive_destination(
    install_destination: &PathBuf, script_name: &String,
) -> Result<String> {
    let parent_path = install_destination
        .parent()
        .ok_or(Error::ResolvePathFailed(install_destination.clone()))?;

    let sdk_path = parent_path.join(PathBuf::from(&script_name));

    Ok(sdk_path
        .to_str()
        .ok_or(Error::ResolvePathFailed(sdk_path.clone()))?
        .to_string())
}

fn download_sdk_archive(destination: &PathBuf) -> Result<PathBuf> {
    let archive_url = get_archive_url()?;

    let archive_name = String::from("as3-sdk.zip");
    let archive_destination =
        get_sdk_archive_destination(destination, &archive_name)?;

    downloader::download_file(&archive_url, &archive_destination)?;

    Ok(PathBuf::from(archive_destination))
}

pub fn install_flex_sdk(destination: &PathBuf) -> Result<()> {
    let archive_path = download_sdk_archive(destination)?;
    let file = File::open(&archive_path).map_err(Error::InvalidArchive)?;

    let mut archive = ZipArchive::new(file)?;
    archive.extract(destination)?;

    fs::remove_file(archive_path).map_err(Error::DeleteArchiveFailed)?;

    Ok(())
}
