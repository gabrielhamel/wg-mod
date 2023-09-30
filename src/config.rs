use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to find the user home directory")]
    UserHomeError,
}

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub wg_mod_home: PathBuf,
}

fn get_tool_home() -> Result<PathBuf, Error> {
    let user_path: std::path::PathBuf =
        home::home_dir().ok_or(Error::UserHomeError)?;
    let wg_tool_path = user_path.join(".wg-mod");
    Ok(wg_tool_path)
}

impl Configs {
    pub fn load() -> Result<Self, Error> {
        Ok(Configs {
            wg_mod_home: get_tool_home()?,
        })
    }
}
