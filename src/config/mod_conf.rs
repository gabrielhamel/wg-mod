use serde_derive::{Deserialize, Serialize};
use std::fmt::Error;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModConf {
    #[serde(rename = "id")]
    pub package_name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
}

impl ModConf {
    pub fn write_json_to_file(
        &self, file_path: &PathBuf,
    ) -> Result<(), io::Error> {
        let file = std::fs::File::create(file_path)?;

        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn write_xml_to_file(
        &self, file_path: &PathBuf,
    ) -> Result<(), io::Error> {
        let file = std::fs::File::create(file_path)?;

        let _ = serde_xml_rs::to_writer(file, self);
        Ok(())
    }

    pub fn from_file(filename: &PathBuf) -> Result<ModConf, io::Error> {
        let file = std::fs::File::open(filename)?;
        Ok(serde_json::from_reader(file)?)
    }
}
