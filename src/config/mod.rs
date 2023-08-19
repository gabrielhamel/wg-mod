pub mod error;

// use self::error::ConfigError;
// use config::Config;
// use serde_derive::Deserialize;

// #[derive(Debug, Deserialize)]
// pub struct Configs {
//     pub wg_mod_home: String,
// }

// fn get_tool_home() -> Result<String, ConfigError> {
//     let user_path: std::path::PathBuf = home::home_dir().ok_or(ConfigError::UserHomeNotFound)?;
//     let wg_tool_path = user_path.join(".wg-mod");
//     let wg_tool_str = wg_tool_path.to_str().ok_or(ConfigError::WgToolHomeInvalid)?;
//     Ok(String::from(wg_tool_str))
// }

// fn model() -> Result<Config, ConfigError> {
//     let config = Config::builder()
//         // .add_source(File::with_name("config.toml").required(false))
//         .set_default("wg_mod_home", get_tool_home()?)?
//         .build()?;
//     Ok(config)
// }

// impl Configs {
//     pub fn load() -> Result<Self, ConfigError> {
//         let model = model()?;
//         let configs = model.try_deserialize()?;

//         Ok(configs)
//     }
// }
