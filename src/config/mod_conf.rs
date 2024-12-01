use crate::utils::file_template;
use crate::utils::file_template::write_template;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::io;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to create this template file")]
    FileTemplateError(#[from] file_template::Error),
}

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

    pub fn export_mod_meta(
        &self, filepath: &PathBuf, filename: &str,
    ) -> Result<(), Error> {
        write_template(
            &filepath,
            filename,
            "<root>
  <id>{{package_name}}</id>
  <version>{{version}}</version>
  <name>{{name}}</name>
  <description>{{description}}</description>
</root>
 ",
            &json!({
                "package_name": self.package_name,
                "version": self.version,
                "name": self.name,
                "description": self.description
            }),
        )?;

        Ok(())
    }

    pub fn from_file(filename: &PathBuf) -> Result<ModConf, io::Error> {
        let file = std::fs::File::open(filename)?;
        Ok(serde_json::from_reader(file)?)
    }
}
