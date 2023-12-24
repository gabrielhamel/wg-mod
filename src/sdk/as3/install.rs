use crate::utils::convert_to_absolute_path::ConvertAbsolutePathError;
use crate::utils::downloader::{download_file, DownloadError};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use zip::result::ZipError;
use zip::ZipArchive;

#[derive(thiserror::Error, Debug)]
pub enum AS3InstallError {
    #[error("No action script sdk available for this platform")]
    PlatformNotSupportedError,

    #[error("Unable to get download directory")]
    ConvertAbsolutePathError(#[from] ConvertAbsolutePathError),

    #[error("SDK download failed")]
    DownloadSdkError(#[from] DownloadError),

    #[error("Provided path is invalid")]
    PathError,

    #[error("Invalid file downloaded")]
    InvalidArchive(std::io::Error),

    #[error("Invalid Zip operation")]
    InvalidZipOperation(#[from] ZipError),

    #[error("Unable to delete archive")]
    RemoveArchiveError(std::io::Error),
}

fn get_archive_url() -> Result<String, AS3InstallError> {
    let os = match std::env::consts::OS {
        | "macos" => Ok("macos"),
        | "windows" => Ok("windows"),
        | _ => Err(AS3InstallError::PlatformNotSupportedError),
    }?;

    Ok(format!(
        "https://wg-mod.s3.eu-west-3.amazonaws.com/apache-flex-{os}.zip"
    ))
}

fn get_sdk_archive_destination(
    install_destination: &PathBuf, script_name: &String,
) -> Result<String, AS3InstallError> {
    Ok(install_destination
        .parent()
        .ok_or(AS3InstallError::PathError)?
        .join(PathBuf::from(&script_name))
        .to_str()
        .ok_or(AS3InstallError::PathError)?
        .to_string())
}

fn download_sdk_archive(
    destination: &PathBuf,
) -> Result<PathBuf, AS3InstallError> {
    let archive_url = get_archive_url()?;

    let archive_name = String::from("as3-sdk.zip");
    let archive_destination =
        get_sdk_archive_destination(destination, &archive_name)?;

    download_file(&archive_url, &archive_destination)?;

    Ok(PathBuf::from(archive_destination))
}

pub fn install_flex_sdk(destination: &PathBuf) -> Result<(), AS3InstallError> {
    let archive_path = download_sdk_archive(destination)?;
    let file =
        File::open(&archive_path).map_err(AS3InstallError::InvalidArchive)?;

    let mut archive = ZipArchive::new(file)?;
    archive.extract(destination)?;

    fs::remove_file(archive_path)
        .map_err(AS3InstallError::RemoveArchiveError)?;

    Ok(())
}
