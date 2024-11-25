mod settings;

use crate::config::settings::Settings;
use crate::sdk::as3::AS3;
use crate::sdk::asconfigc::ASConfigc;
use crate::sdk::conda::environment::CondaEnvironment;
use crate::sdk::conda::Conda;
use crate::sdk::game_sources::GameSources;
use crate::sdk::nvm::BoxedNVM;
use crate::sdk::{as3, asconfigc, conda, game_sources, nvm};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to find the user home directory")]
    UserHomeError,

    #[error("Unable to load game sources\n{0}")]
    GameSourcesError(#[from] game_sources::Error),

    #[error("Unable to load Conda\n{0}")]
    CondaError(#[from] conda::Error),

    #[error("Unable to load AS3\n{0}")]
    AS3Error(#[from] as3::Error),

    #[error("Unable to load settings\n{0}")]
    SettingsError(String),

    #[error("Invalid json\n{0}")]
    SettingsParseError(#[from] serde_json::Error),

    #[error("NVM error")]
    NVMError(#[from] nvm::Error),

    #[error("ASConfigc loading error")]
    ASConfigcError(#[from] asconfigc::Error),
}

type Result<T> = result::Result<T, Error>;

pub struct Configs {
    pub wg_mod_home: PathBuf,
    pub game_sources: GameSources,
    pub conda_environment: CondaEnvironment,
    pub as3: AS3,
    pub asconfigc: ASConfigc,
    pub settings: Settings,
}

fn get_tool_home() -> Result<PathBuf> {
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

        Ok(Configs {
            game_sources,
            wg_mod_home,
            conda_environment,
            as3,
            asconfigc,
            settings,
        })
    }

    pub fn save_settings(&self) -> Result<()> {
        let settings_file = self.wg_mod_home.join("settings.json");
        let file = File::create(settings_file).map_err(|e| {
            Error::SettingsError(format!("Unable to open settings file: {e}"))
        })?;

        serde_json::to_writer_pretty(file, &self.settings)?;

        Ok(())
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

fn create_default_settings_file(path: &PathBuf) -> Result<()> {
    let mut file = File::create(path).map_err(|e| {
        Error::SettingsError(format!("Unable to create settings file: {e}"))
    })?;

    file.write("{}".as_ref()).map_err(|e| {
        Error::SettingsError(format!("Unable to write in settings file: {e}"))
    })?;

    Ok(())
}

fn load_settings(wg_mod_home: &PathBuf) -> Result<Settings> {
    let settings_file = wg_mod_home.join("settings.json");

    if !settings_file.exists() {
        create_default_settings_file(&settings_file)?
    };

    let file = File::open(&settings_file).map_err(|e| {
        Error::SettingsError(format!("Unable to open settings file: {e}"))
    })?;

    let settings: Settings = serde_json::from_reader(file)?;

    Ok(settings)
}
