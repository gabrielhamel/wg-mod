pub mod asconfig_json;
pub mod mod_conf;
pub mod settings;

use crate::builder::flash_lib;
use crate::builder::flash_lib::build_flash_client_lib;
use crate::config::settings::Settings;
use crate::sdk::as3::AS3;
use crate::sdk::asconfigc::ASConfigc;
use crate::sdk::conda::environment::CondaEnvironment;
use crate::sdk::conda::Conda;
use crate::sdk::game_client::GameClient;
use crate::sdk::game_sources::GameSources;
use crate::sdk::nvm::BoxedNVM;
use crate::sdk::{as3, asconfigc, conda, game_sources, nvm, Installable};
use crate::utils;
use inquire::InquireError;
use std::env::VarError;
use std::path::PathBuf;
use std::result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to find the user home directory")]
    UserHomeError,

    #[error("Terminal prompt error: {0}")]
    PromptError(#[from] InquireError),

    #[error("Unable to load game sources: {0}")]
    GameSourcesError(#[from] game_sources::Error),

    #[error("Unable to load game client")]
    GameClientError,

    #[error("Unable to build client flash lib: {0}")]
    GameClientLibError(#[from] flash_lib::Error),

    #[error("Unable to load Conda: {0}")]
    CondaError(#[from] conda::Error),

    #[error("Unable to load AS3: {0}")]
    AS3Error(#[from] as3::Error),

    #[error("Unable to load settings: {0}")]
    SettingsError(#[from] settings::Error),

    #[error("NVM error")]
    NVMError(#[from] nvm::Error),

    #[error("ASConfigc loading error")]
    ASConfigcError(#[from] asconfigc::Error),

    #[error("Failed to compose string: {0}")]
    StringError(#[from] std::fmt::Error),

    #[error("Failed convertion: {0}")]
    ConvertionError(#[from] utils::convert_pathbuf_to_string::Error),

    #[error("Unable to read current user name")]
    Environment(#[from] VarError),
}

type Result<T> = result::Result<T, Error>;

pub struct Configs {
    pub wg_mod_home: PathBuf,
    pub game_sources: GameSources,
    pub game_client: Option<GameClient>,
    pub conda_environment: CondaEnvironment,
    pub as3: AS3,
    pub asconfigc: ASConfigc,
    pub settings: Settings,
}

pub fn get_tool_home() -> Result<PathBuf> {
    let user_path: PathBuf = home::home_dir().ok_or(Error::UserHomeError)?;
    let wg_tool_path = user_path.join(".wg-mod");
    Ok(wg_tool_path)
}

impl Configs {
    pub fn load() -> Result<Self> {
        let wg_mod_home = get_tool_home()?;
        let game_sources = load_game_sources(&wg_mod_home)?;
        let conda_environment = load_conda_environment(&wg_mod_home)?;
        let as3 = load_as3(&wg_mod_home)?;
        let settings = load_settings(&wg_mod_home)?;
        let asconfigc = load_asconfigc(&wg_mod_home)?;
        let game_client = load_game_client(&settings);
        build_flash_client_lib(&wg_mod_home)?;

        Ok(Configs {
            game_sources,
            game_client,
            wg_mod_home,
            conda_environment,
            as3,
            asconfigc,
            settings,
        })
    }
}

fn load_asconfigc(wg_mod_home: &PathBuf) -> Result<ASConfigc> {
    let nvm = load_nvm(&wg_mod_home)?;
    let asconfigc = asconfigc::load_asconfigc(nvm)?;

    Ok(asconfigc)
}

fn load_game_sources(wg_mod_home: &PathBuf) -> Result<GameSources> {
    let game_sources_path = wg_mod_home.join("wot-src");
    let game_sources = GameSources::load(&game_sources_path)?;

    Ok(game_sources)
}

fn load_conda(wg_mod_home: &PathBuf) -> Result<Conda> {
    let conda_path = wg_mod_home.join("conda");
    let conda = conda::load_conda(&conda_path)?;

    Ok(conda)
}

fn load_as3(wg_mod_home: &PathBuf) -> Result<AS3> {
    let as3_path = wg_mod_home.join("as3");
    let as3 = as3::load_as3(&as3_path)?;

    Ok(as3)
}

fn load_nvm(wg_mod_home: &PathBuf) -> Result<BoxedNVM> {
    let nvm_path = wg_mod_home.join("nvm");
    let nvm = nvm::load_nvm(&nvm_path)?;

    Ok(nvm)
}

fn load_conda_environment(wg_mod_home: &PathBuf) -> Result<CondaEnvironment> {
    let conda = load_conda(wg_mod_home)?;

    if !conda.has_environment("wg-mod") {
        println!("Create conda env...");
        conda.create_environment("wg-mod", "2")?;
    }

    Ok(conda.get_environment("wg-mod"))
}

fn load_game_client(settings: &Settings) -> Option<GameClient> {
    if let Some(game_client_path) = &settings.game_client_path {
        Some(GameClient::from(game_client_path))
    } else {
        None
    }
}

fn load_settings(wg_mod_home: &PathBuf) -> Result<Settings> {
    let settings_file_path = wg_mod_home.join("settings.json");
    let mut settings: Settings;

    if !settings_file_path.exists() {
        settings =
            Settings::create_default_settings(settings_file_path.clone());
    } else {
        settings = Settings::from_json_file(&settings_file_path)?;
    }

    settings.verify_game_client_path_validity();
    settings.write_to_json_file()?;

    Ok(settings)
}

fn get_conda(wg_mod_home: &PathBuf) -> Result<Conda> {
    let conda_path = wg_mod_home.join("conda");
    let conda = Conda::from(&conda_path);

    if !conda.is_installed() {
        println!("Installing conda...");
        conda.install().expect("failed conda installation");
    }

    Ok(conda)
}
