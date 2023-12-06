use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum AS3InstallError {
    #[error("No action script sdk available for this platform")]
    PlatformNotSupportedError,
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

fn download_sdk_archive() -> Result<(), AS3InstallError> {
    let archive_url = get_archive_url()?;

    println!("Archive URL: {}", archive_url);

    Ok(())
}

pub fn install_flex_sdk(destination: &PathBuf) -> Result<(), AS3InstallError> {
    let _ = download_sdk_archive()?;

    Ok(())
}
