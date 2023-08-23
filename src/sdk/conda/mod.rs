use crate::{
    config::Configs,
    utils::{downloader::download_file, file_template},
};
use std::path::PathBuf;

pub async fn download() {
    download_file(
        "https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-arm64.sh",
        "Miniconda3-latest-MacOSX-arm64.sh",
    )
    .await
    .unwrap();
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Can't access to configs")]
    ConfigError(#[from] crate::config::Error),

    #[error("Cannot download provided url")]
    DownloadError(#[from] crate::utils::downloader::Error),
}

struct Conda {
    conda_path: PathBuf,
}

impl Conda {
    pub fn default() -> Result<Self, Error> {
        let config = Configs::load()?;
        let conda_path = config.wg_mod_home.join("conda");

        Ok(Self { conda_path })
    }

    pub async fn install() -> Result<(), Error> {
        let arch = match (std::env::consts::OS, std::env::consts::ARCH) {
            | ("macos", "aarch64") => ("MacOSX", "arm64"),
            | ("windows", arch) => ("Windows", arch),
            | (os, arch) => (os, arch),
        };

        println!("{}", std::env::consts::OS);
        println!("{}", std::env::consts::ARCH);

        download_file(
            "https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-arm64.sh",
            "Miniconda3-latest-MacOSX-arm64.sh",
        )
        .await?;

        Ok(())
    }
}
