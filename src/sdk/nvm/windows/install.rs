use crate::new::template::template_nvm_config;
use crate::sdk::nvm::{create_nvm_directory, NVMError};
use crate::utils::convert_pathbuf_to_string::Stringify;
use crate::utils::downloader::download_file;
use std::fs;
use std::path::PathBuf;
use zip_extensions::zip_extract;

pub fn install_nvm_windows(nvm_path: &PathBuf) -> Result<(), NVMError> {
    create_nvm_directory(nvm_path)
        .map_err(|e| NVMError::InstallError(e.to_string()))?;
    let downloaded_file_path = nvm_path.join("nvm.zip");
    let downloaded_file = downloaded_file_path
        .to_string()
        .map_err(|e| NVMError::InstallError(e.to_string()))?;

    // not in 1.1.12 because issue in it: https://github.com/coreybutler/nvm-windows/issues/1068
    download_file(
        "https://github.com/coreybutler/nvm-windows/releases/download/1.1.11/nvm-noinstall.zip",
        &downloaded_file,
    )
        .map_err(|e| NVMError::DownloadError(e.to_string()))?;

    zip_extract(&downloaded_file_path, nvm_path)
        .map_err(|e| NVMError::InstallError(e.to_string()))?;

    fs::remove_file(&downloaded_file_path)
        .map_err(|e| NVMError::InstallError(e.to_string()))?;

    template_nvm_config(nvm_path)
        .map_err(|e| NVMError::InstallError(e.to_string()))?;

    Ok(())
}
