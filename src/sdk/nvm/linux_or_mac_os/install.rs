use crate::sdk::nvm;
use crate::sdk::nvm::create_nvm_directory;
use crate::utils::convert_pathbuf_to_string::Stringify;
use crate::utils::downloader::download_file;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn install_nvm_sdk(nvm_path: &PathBuf) -> nvm::Result<()> {
    create_nvm_directory(nvm_path)
        .map_err(|e| nvm::Error::InstallError(e.to_string()))?;
    let downloaded_file_path = nvm_path.join("install.sh");
    let downloaded_file = downloaded_file_path.to_string()?;

    download_file(
        "https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh",
        &downloaded_file,
    )
    .map_err(|e| nvm::Error::DownloadError(e.to_string()))?;

    let mut command = Command::new("bash");
    command.arg(&downloaded_file).env("NVM_DIR", nvm_path);

    let _ = command
        .output()
        .map_err(|e| nvm::Error::InstallError(e.to_string()))?;

    fs::remove_file(&downloaded_file_path)
        .map_err(|e| nvm::Error::InstallError(e.to_string()))?;

    Ok(())
}
