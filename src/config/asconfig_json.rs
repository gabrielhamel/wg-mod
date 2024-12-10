use crate::config::mod_conf::ModConf;
use serde_derive::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AsconfigcJson {
    #[serde(rename = "config")]
    pub config: String,
    #[serde(rename = "compilerOptions")]
    pub compiler_option: CompilerOption,
    #[serde(rename = "mainClass")]
    pub main_class: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerOption {
    #[serde(rename = "output")]
    pub output: String,
    #[serde(rename = "source-path")]
    pub source_path: Vec<String>,
}

impl AsconfigcJson {
    pub fn write_json_to_file(
        &self, file_path: &PathBuf,
    ) -> Result<(), io::Error> {
        let file = std::fs::File::create(file_path)?;

        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn from_file(filename: &PathBuf) -> Result<AsconfigcJson, io::Error> {
        let file = std::fs::File::open(filename)?;
        Ok(serde_json::from_reader(file)?)
    }
}
