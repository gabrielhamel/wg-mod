mod settings;

use crate::config::settings::Settings;
use crate::sdk::as3::{self, AS3Error, AS3};
use crate::sdk::conda::environment::CondaEnvironment;
use crate::sdk::conda::Conda;
use crate::sdk::game_sources::{GameSources, GameSourcesError};
use crate::sdk::{conda, Installable};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::sdk::node::{Node};
use crate::sdk::nvm::linuxOrMacOS::LinuxOrMacOsNVM;
use crate::sdk::nvm::{ NVM};
use crate::sdk::nvm::windows::WindowsNVM;

#[derive(thiserror::Error, Debug)]
pub enum ConfigsError {
    #[error("Unable to find the user home directory")]
    UserHomeError,

    #[error("Unable to load game sources\n{0}")]
    GameSourcesError(#[from] GameSourcesError),

    #[error("Unable to load Conda\n{0}")]
    CondaError(#[from] conda::CondaError),

    #[error("Unable to load AS3\n{0}")]
    AS3Error(#[from] as3::AS3Error),

    #[error("Unable to load settings\n{0}")]
    SettingsError(String),

    #[error("Invalid json\n{0}")]
    SettingsParseError(#[from] serde_json::Error),
}

pub struct Configs {
    pub wg_mod_home: PathBuf,
    pub game_sources: GameSources,
    pub conda_environment: CondaEnvironment,
    pub as3: AS3,
    pub nvm: Box<dyn NVM>,
    pub node: Node,
    pub settings: Settings,
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
        let as3 = load_as3(&wg_mod_home)?;
        let nvm = load_nvm(&wg_mod_home)?;
        let node = load_node(&wg_mod_home, &nvm)?;
        let settings = load_settings(&wg_mod_home)?;

        Ok(Configs {
            game_sources,
            wg_mod_home,
            conda_environment,
            as3,
            nvm,
            node,
            settings,
        })
    }

    pub fn save_settings(&self) -> Result<(), ConfigsError> {
        let settings_file = self.wg_mod_home.join("settings.json");
        let file = File::create(settings_file).map_err(|e| {
            ConfigsError::SettingsError(format!(
                "Unable to open settings file: {e}"
            ))
        })?;

        serde_json::to_writer_pretty(file, &self.settings)?;

        Ok(())
    }
}

fn create_default_settings_file(path: &PathBuf) -> Result<(), ConfigsError> {
    let mut file = File::create(path).map_err(|e| {
        ConfigsError::SettingsError(format!(
            "Unable to create settings file: {e}"
        ))
    })?;

    file.write("{}".as_ref()).map_err(|e| {
        ConfigsError::SettingsError(format!(
            "Unable to write in settings file: {e}"
        ))
    })?;

    Ok(())
}

fn load_settings(wg_mod_home: &PathBuf) -> Result<Settings, ConfigsError> {
    let settings_file = wg_mod_home.join("settings.json");

    if !settings_file.exists() {
        create_default_settings_file(&settings_file)?
    };

    let file = File::open(&settings_file).map_err(|e| {
        ConfigsError::SettingsError(format!(
            "Unable to open settings file: {e}"
        ))
    })?;

    let settings: Settings = serde_json::from_reader(file)?;

    Ok(settings)
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

    if !conda.is_installed() {
        println!("Installing conda...");
        conda.install().expect("");
    }

    Ok(conda)
}

fn load_as3(wg_mod_home: &PathBuf) -> Result<AS3, AS3Error> {
    let as3_path = wg_mod_home.join("as3");
    let as3 = AS3::from(as3_path);

    if !as3.is_installed() {
        println!("Installing action script SDK...");
        as3.install().expect("");
    }
    Ok(as3)
}

fn load_nvm(wg_mod_home: &PathBuf) -> Result<Box<dyn NVM>, ConfigsError> {
    let nvm_path = wg_mod_home.join("nvm");
    let nvm : Box<dyn NVM> = if cfg!(target_os = "windows") {
        Box::new(WindowsNVM::from(nvm_path))
    }else {
        Box::new(LinuxOrMacOsNVM::from(nvm_path))
    };

    nvm.install().expect("");

    Ok(nvm)
}

fn load_node(wg_mod_home: &PathBuf, nvm: &Box<dyn NVM>) -> Result<Node, ConfigsError> {
    let node_path = wg_mod_home.join("node");
    let node = Node::new(node_path);

    node.install(nvm).expect("");

    Ok(node)
}
