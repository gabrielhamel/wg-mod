use crate::sdk::conda;
use crate::sdk::conda::environment::CondaEnvironment;
use crate::sdk::conda::Conda;
use crate::sdk::game_sources::{GameSources, GameSourcesError};
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ConfigsError {
    #[error("Unable to find the user home directory")]
    UserHomeError,

    #[error("Unable to load game sources\n{0}")]
    GameSourcesError(#[from] GameSourcesError),

    #[error("Unable to load Conda\n{0}")]
    CondaError(#[from] conda::CondaError),
}

pub struct Configs {
    pub wg_mod_home: PathBuf,
    pub game_sources: GameSources,
    pub conda_environment: CondaEnvironment,
}

fn get_tool_home() -> Result<PathBuf, ConfigsError> {
    let user_path: std::path::PathBuf =
        home::home_dir().ok_or(ConfigsError::UserHomeError)?;
    let wg_tool_path = user_path.join(".wg-mod");
    Ok(wg_tool_path)
}

impl Configs {
    pub fn load() -> Result<Self, ConfigsError> {
        let wg_mod_home = get_tool_home()?;
        let game_sources = load_game_sources(&wg_mod_home)?;
        let conda_environment = load_conda_environment(&wg_mod_home)?;

        Ok(Configs {
            game_sources,
            wg_mod_home,
            conda_environment,
        })
    }
}

fn load_game_sources(
    wg_mod_home: &PathBuf,
) -> Result<GameSources, ConfigsError> {
    let game_sources_path = wg_mod_home.join("wot-src");
    let game_sources = GameSources::load(&game_sources_path)?;

    Ok(game_sources)
}

fn load_conda_environment(
    wg_mod_home: &PathBuf,
) -> Result<CondaEnvironment, ConfigsError> {
    let conda = get_conda(wg_mod_home)?;

    if !conda.has_environment("wg-mod") {
        println!("Create conda env...");
        conda.create_environment("wg-mod", "2")?;
    }

    Ok(conda.get_environment("wg-mod"))
}

fn get_conda(wg_mod_home: &PathBuf) -> Result<Conda, ConfigsError> {
    let conda_path = wg_mod_home.join("conda");
    let conda = Conda::from(conda_path);

    if !conda.is_installed().expect("") {
        println!("Installing conda...");
        conda.install().expect("");
    }

    Ok(conda)
}
