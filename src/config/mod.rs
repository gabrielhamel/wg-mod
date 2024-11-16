mod settings;

use crate::config::settings::Settings;
use crate::sdk::as3::{AS3Error, AS3};
use crate::sdk::asconfigc::ASConfigc;
use crate::sdk::conda::environment::CondaEnvironment;
use crate::sdk::conda::Conda;
use crate::sdk::game_sources::{GameSources, GameSourcesError};
use crate::sdk::nvm::linux_or_mac_os::LinuxOrMacOsNVM;
use crate::sdk::nvm::windows::WindowsNVM;
use crate::sdk::nvm::{NVMError, NVM};
use crate::sdk::{conda, Installable};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ConfigsError {
    #[error("Unable to find the user home directory")]
    UserHomeError,

    #[error("Unable to load game sources\n{0}")]
    GameSourcesError(#[from] GameSourcesError),

    #[error("Unable to load Conda\n{0}")]
    CondaError(#[from] conda::CondaError),

    #[error("Unable to load AS3\n{0}")]
    AS3Error(#[from] AS3Error),

    #[error("Unable to load settings\n{0}")]
    SettingsError(String),

    #[error("Invalid json\n{0}")]
    SettingsParseError(#[from] serde_json::Error),

    #[error("Nvm config failed")]
    ConfigNVMError(#[from] NVMError),

    #[error("ASConfig loading error")]
    ASConfigError,
}

pub struct Configs {
    pub wg_mod_home: PathBuf,
    pub game_sources: GameSources,
    pub conda_environment: CondaEnvironment,
    pub as3: AS3,
    pub asconfigc: ASConfigc,
    pub settings: Settings,
}

fn get_tool_home() -> Result<PathBuf, ConfigsError> {
    let user_path: PathBuf =
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
        let settings = load_settings(&wg_mod_home)?;
        let asconfigc = load_asconfigc(&wg_mod_home)?;

        let res = asconfigc
            .exec(vec!["--help"], vec![])
            .expect("ASCONFIGERROR");
        println!(
            "{:?} {:?} {:?}",
            res.status,
            String::from_utf8_lossy(&res.stdout),
            String::from_utf8_lossy(&res.stderr)
        );

        Ok(Configs {
            game_sources,
            wg_mod_home,
            conda_environment,
            as3,
            asconfigc,
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

fn load_asconfigc(wg_mod_home: &PathBuf) -> Result<ASConfigc, ConfigsError> {
    let nvm = load_nvm(&wg_mod_home)?;
    let node = nvm.get_node()?;
    let npm = node.get_npm();
    let asconfigc = ASConfigc::from(npm);

    if !asconfigc.is_installed() {
        asconfigc.install().map_err(|e| {
            eprintln!("{}", e);
            ConfigsError::ASConfigError
        })?;
    }

    Ok(asconfigc)
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
        conda.install().expect("failed conda installation");
    }

    Ok(conda)
}

fn load_as3(wg_mod_home: &PathBuf) -> Result<AS3, AS3Error> {
    let as3_path = wg_mod_home.join("as3");
    let as3 = AS3::from(as3_path);

    if !as3.is_installed() {
        println!("Installing action script SDK...");
        as3.install().expect("failed s3 installation");
    }
    Ok(as3)
}

fn load_nvm(wg_mod_home: &PathBuf) -> Result<Box<dyn NVM>, ConfigsError> {
    let nvm_path = wg_mod_home.join("nvm");
    let nvm_destination = nvm_path.clone();

    let nvm_installer: Box<dyn Installable> = if cfg!(target_os = "windows") {
        Box::new(WindowsNVM::from(nvm_destination))
    } else {
        Box::new(LinuxOrMacOsNVM::from(nvm_destination))
    };

    if !nvm_installer.is_installed() {
        println!("Install nvm ...");
        nvm_installer.install().expect("failed ton install nvm");
    }

    let nvm: Box<dyn NVM> = if cfg!(target_os = "windows") {
        Box::new(WindowsNVM::from(nvm_path))
    } else {
        Box::new(LinuxOrMacOsNVM::from(nvm_path))
    };

    Ok(nvm)
}
