use crate::sdk::conda::Error;
use crate::utils::downloader::download_file;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::{Command, Output};

pub async fn install_conda(destination: &PathBuf) -> Result<(), Error> {
    create_dir_all(destination).map_err(Error::CreateCondaDirectory)?;

    let install_script_name = get_install_script_name();
    let install_script_destination =
        get_script_destination(destination, &install_script_name)?;

    let url =
        format!("https://repo.anaconda.com/miniconda/{install_script_name}");

    download_file(&url, install_script_destination.as_str()).await?;

    let install_destination = destination.to_str().ok_or(Error::PathError)?;
    if cfg!(target_os = "windows") {
        install_on_windows(&install_script_destination, install_destination)?
    } else {
        install_on_linux_and_macos(
            &install_script_destination,
            install_destination,
        )?
    };

    Ok(())
}

fn get_install_script_name() -> String {
    let enforce_x86_arch_on_macos = "x86_64";

    let (os, arch, extension) =
        match (std::env::consts::OS, std::env::consts::ARCH) {
            | ("macos", "aarch64") => ("MacOSX", enforce_x86_arch_on_macos, "sh"),
            | ("macos", arch) => ("MacOSX", arch, "sh"),
            | ("windows", arch) => ("Windows", arch, "exe"),
            | (os, arch) => (os, arch, "sh"),
        };

    format!("Miniconda3-latest-{os}-{arch}.{extension}")
}

fn get_script_destination(
    install_destination: &PathBuf, script_name: &String,
) -> Result<String, Error> {
    Ok(install_destination
        .parent()
        .ok_or(Error::PathError)?
        .join(PathBuf::from(&script_name))
        .to_str()
        .ok_or(Error::PathError)?
        .to_string())
}

fn install_on_windows(
    script_location: &String, conda_path: &str,
) -> Result<Output, Error> {
    Ok(Command::new(script_location)
        .args(["/S", &format!("/D={conda_path}")])
        .output()
        .map_err(Error::InstallError)?)
}

fn install_on_linux_and_macos(
    script_location: &String, conda_path: &str,
) -> Result<Output, Error> {
    Ok(Command::new("sh")
        .args([script_location, "-p", conda_path, "-b", "-u"])
        .output()
        .map_err(Error::InstallError)?)
}
