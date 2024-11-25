use crate::sdk::conda;
use crate::utils::downloader::download_file;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::{fs, result};

type Result<T> = result::Result<T, conda::Error>;

pub fn install_conda(destination: &PathBuf) -> Result<()> {
    fs::create_dir_all(destination)
        .map_err(conda::Error::CreateCondaDirectory)?;

    let install_script_name = get_install_script_name();
    let install_script_destination =
        get_script_destination(destination, &install_script_name)?;

    let url =
        format!("https://repo.anaconda.com/miniconda/{install_script_name}");
    download_file(&url, install_script_destination.as_str())?;

    let install_destination =
        destination.to_str().ok_or(conda::Error::PathError)?;

    let install_result = if cfg!(target_os = "windows") {
        install_on_windows(&install_script_destination, install_destination)?
    } else {
        install_on_linux_and_macos(
            &install_script_destination,
            install_destination,
        )?
    };

    if !install_result.status.success() {
        eprintln!("status: {}", install_result.status);
        eprintln!(
            "stdout: {}",
            String::from_utf8_lossy(&install_result.stdout)
        );
        eprintln!(
            "stderr: {}",
            String::from_utf8_lossy(&install_result.stderr)
        );
        return Err(conda::Error::NotInstalledError);
    }

    fs::remove_file(install_script_destination)
        .map_err(conda::Error::InstallError)?;

    Ok(())
}

fn get_install_script_name() -> String {
    let enforce_x86_arch_on_macos = "x86_64";

    let (os, arch, extension) =
        match (std::env::consts::OS, std::env::consts::ARCH) {
            | ("macos", "aarch64") => {
                ("MacOSX", enforce_x86_arch_on_macos, "sh")
            },
            | ("macos", arch) => ("MacOSX", arch, "sh"),
            | ("windows", arch) => ("Windows", arch, "exe"),
            | ("linux", arch) => ("Linux", arch, "sh"),
            | (os, arch) => (os, arch, "sh"),
        };

    format!("Miniconda3-latest-{os}-{arch}.{extension}")
}

fn get_script_destination(
    install_destination: &PathBuf, script_name: &String,
) -> Result<String> {
    Ok(install_destination
        .parent()
        .ok_or(conda::Error::PathError)?
        .join(PathBuf::from(&script_name))
        .to_str()
        .ok_or(conda::Error::PathError)?
        .to_string())
}

fn install_on_windows(
    script_location: &String, conda_path: &str,
) -> Result<Output> {
    Ok(Command::new(script_location)
        .args(["/S", &format!("/D={conda_path}")])
        .output()
        .map_err(conda::Error::InstallError)?)
}

fn install_on_linux_and_macos(
    script_location: &String, conda_path: &str,
) -> Result<Output> {
    Ok(Command::new("sh")
        .args([script_location, "-p", conda_path, "-b", "-u"])
        .output()
        .map_err(conda::Error::InstallError)?)
}
