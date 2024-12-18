use crate::config::settings::load_settings;
use crate::utils::convert_pathbuf_to_string::Stringify;
use crate::utils::copy_directory::copy_directory;
use crate::utils::extract_archive;
use crate::utils::extract_archive::extract_archive;
use regex::Regex;
use std::fs::{create_dir_all, read_dir, remove_dir_all, DirEntry, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;
use zip::unstable::write::FileOptionsExt;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Failed to get game client flash lib: {0}")]
    LibError(String),
    #[error("Failed to extract archive: {0}")]
    ExtractError(#[from] extract_archive::Error),
    #[error("Failed to build lib: {0}")]
    BuildError(String),
    #[error("Value is null: {0}")]
    NullError(String),
    #[error("Installation failed: {0}")]
    InstallationError(String),
    #[error("Convertion failed: {0}")]
    PatternError(#[from] regex::Error),
}

pub struct GameFlashLib {
    game_flash_lib: PathBuf,
}

impl From<PathBuf> for GameFlashLib {
    fn from(value: PathBuf) -> Self {
        Self {
            game_flash_lib: value,
        }
    }
}

impl GameFlashLib {
    fn get_flash_archive_path(
        &self, game_client_path: &PathBuf, file_identifier: String,
    ) -> PathBuf {
        game_client_path
            .join(format!("res/packages/gui-part{}.pkg", file_identifier))
    }

    fn is_present(&self) -> bool {
        self.game_flash_lib.exists()
    }

    fn build(&self) -> Result<(), Error> {
        println!("Building game flash lib...");
        let tmp_dir =
            tempdir().map_err(|e| Error::BuildError(e.to_string()))?;

        let settings =
            load_settings().map_err(|e| Error::BuildError(e.to_string()))?;
        let game_client_path = settings
            .game_client_path
            .ok_or(Error::NullError("game_client_path".to_string()))?;

        let package_path = game_client_path.join("res/packages/");
        let archive_list = read_dir(package_path)
            .map_err(|e| Error::BuildError(e.to_string()))?;

        let pattern = Regex::new(r"^.*gui-part[0-9].pkg?")?;
        for archive in archive_list.flatten() {
            let dir_item_path = archive.path();
            let str = dir_item_path.to_str().ok_or(Error::BuildError(
                "failed to convert path to string".to_string(),
            ))?;
            if pattern.is_match(str) {
                extract_archive(&dir_item_path, &tmp_dir.path().to_path_buf())
                    .map_err(|e| Error::BuildError(e.to_string()))?;
            }
        }

        let inside_archive_path =
            tmp_dir.path().to_path_buf().join("gui/flash/swc/");
        copy_directory(&inside_archive_path, &self.game_flash_lib)
            .map_err(|e| Error::BuildError(e.to_string()))?;

        tmp_dir.close().ok();
        Ok(())
    }
}

pub fn build_flash_client_lib(
    wg_mod_home: &PathBuf,
) -> Result<GameFlashLib, Error> {
    let game_flash_lib_path = wg_mod_home.join("flash_lib");
    let game_flash_lib = GameFlashLib::from(game_flash_lib_path);

    if game_flash_lib.game_flash_lib.exists() {
        remove_dir_all(&game_flash_lib.game_flash_lib)
            .map_err(|e| Error::BuildError(e.to_string()))?;
    }

    create_dir_all(&game_flash_lib.game_flash_lib)
        .map_err(|e| Error::BuildError(e.to_string()))?;

    game_flash_lib
        .build()
        .map_err(|e| Error::BuildError(e.to_string()))?;

    Ok(game_flash_lib)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_flash_archive_path() {
        let tmp_dir = tempdir().unwrap();

        let game_client_lib_path =
            GameFlashLib::from(tmp_dir.path().to_path_buf());

        let flash_file_path = game_client_lib_path.get_flash_archive_path(
            &tmp_dir.path().to_path_buf(),
            "1".to_string(),
        );

        assert_eq!(
            flash_file_path,
            tmp_dir
                .path()
                .to_path_buf()
                .join("res/packages/gui-part1.pkg")
        );
    }
}
