mod flash;
mod python;

use crate::builder::flash::FlashBuilder;
use crate::builder::python::PythonBuilder;
use crate::config;
use crate::config::asconfig_json::AsconfigcJson;
use crate::config::mod_conf::ModConf;
use crate::config::{get_tool_home, mod_conf};
use crate::sdk::flash_lib;
use crate::sdk::flash_lib::extract_flash_client_lib;
use crate::utils::convert_pathbuf_to_string::Stringify;
use crate::utils::convert_to_absolute_path::convert_to_absolute_path;
use crate::utils::{convert_pathbuf_to_string, convert_to_absolute_path};
use convert_case::{Case, Casing};
use inquire::InquireError;
use std::path::PathBuf;
use std::{fs, io, result};
use zip::result::ZipError;
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip_extensions::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to use the python mod builder\n{0}")]
    PythonBuilderError(#[from] python::Error),

    #[error("Failed to use the flash mod builder\n{0}")]
    FlashBuilderError(#[from] flash::Error),

    #[error("Copy directory failed\n{0}")]
    CopyDirectoryError(#[from] fs_extra::error::Error),

    #[error("Glob error\n{0}")]
    GlobError(#[from] glob::GlobError),

    #[error("The path \"{0}\" isn't a mod folder")]
    BadModFolderError(PathBuf),

    #[error("Path error: {0}")]
    PathError(String),

    #[error("Unable to write mod archive\n{0}")]
    ZipWriteError(#[from] ZipError),

    #[error("Unable to get the absolute path of the archive: {0}")]
    ConvertAbsolutePathError(#[from] convert_to_absolute_path::Error),

    #[error("Unable to get the absolute path of the archive: {0}")]
    ConvertPathToStringError(#[from] convert_pathbuf_to_string::Error),

    #[error("Manage config file: {0}")]
    ConfigFileError(#[from] io::Error),

    #[error("Unable to write mod config: {0}")]
    ModConfigFileError(#[from] mod_conf::Error),

    #[error("Failed to get config: {0}")]
    ConfigError(#[from] config::Error),

    #[error("Failed prompt main class_name: {0}")]
    PromptError(#[from] InquireError),

    #[error("Failed build game client flash lib: {0}")]
    BuildFlashLibError(#[from] flash_lib::Error),
}

type Result<T> = result::Result<T, Error>;

pub struct ModBuilder {
    python_builder: PythonBuilder,
    flash_builder: FlashBuilder,
    mod_path: PathBuf,
    target_path: PathBuf,
    build_path: PathBuf,
}

impl ModBuilder {
    pub fn new(mod_path: PathBuf) -> Result<Self> {
        let python_builder = PythonBuilder::new()?;
        let flash_builder = FlashBuilder::new()?;
        let target_path = mod_path.join("target");
        let build_path = target_path.join("build");

        Ok(Self {
            python_builder,
            flash_builder,
            mod_path,
            target_path,
            build_path,
        })
    }

    fn clean_target_directory(&self) -> Result<()> {
        let _ = fs::remove_dir_all(&self.target_path);

        Ok(())
    }

    fn build_python_src(&self) -> Result<()> {
        let python_sources = self.mod_path.join("scripts");
        let python_build_destination =
            self.build_path.join("res/scripts/client/gui/mods");

        self.python_builder
            .build(&python_sources, &python_build_destination)?;

        Ok(())
    }

    fn copy_meta_file(&self) -> Result<()> {
        let meta_path = self.mod_path.join("mod.json");
        let mod_conf = ModConf::from_file(&meta_path)?;
        mod_conf.export_mod_meta(&self.build_path, "meta.xml")?;

        Ok(())
    }
    fn build_flash_src(&self) -> Result<()> {
        let flash_sources = self.mod_path.join("ui");
        let flash_build_destination = self.build_path.join("res/gui/flash");

        if !flash_sources.exists() {
            Err(Error::PathError(format!(
                "Flash source does not exist: {:?}",
                flash_sources
            )))?
        }

        let wg_home = get_tool_home()?;
        extract_flash_client_lib(&wg_home)?;

        self.update_asconfigc_json(&flash_sources)?;

        self.flash_builder
            .build(&flash_sources, &flash_build_destination)?;

        Ok(())
    }

    fn update_asconfigc_json(&self, flash_sources: &PathBuf) -> Result<()> {
        let asconfigc_json_path = flash_sources.join("asconfig.json");
        let mut asconfigc = AsconfigcJson::from_file(&asconfigc_json_path)?;

        let main_class_string_path = asconfigc.main_class.replace(".", "/");

        let prompt_class_string =
            self.prompt_main_class(main_class_string_path.as_str())?;

        let prompt_class_with_extention = if prompt_class_string.contains(".as")
        {
            &prompt_class_string
        } else {
            &format!("{}.as", &prompt_class_string)
        };
        let prompt_class_globale_path = self
            .mod_path
            .join("ui")
            .join("src")
            .join(&prompt_class_with_extention);
        if !prompt_class_globale_path.exists() {
            let error_message = format!("Prompt class path given ({:?}) does not exist (File not found)", prompt_class_globale_path.to_string());
            Err(Error::PathError(error_message))?
        }

        let prompt_class_without_extention =
            if prompt_class_string.contains(".as") {
                &prompt_class_string.replace(".as", "")
            } else {
                &prompt_class_string
            };
        let prompt_class = prompt_class_without_extention.replace("/", ".");
        if !prompt_class.eq(asconfigc.main_class.as_str()) {
            asconfigc.main_class = prompt_class;
        }

        let meta_path = self.mod_path.join("mod.json");
        let mod_conf = ModConf::from_file(&meta_path)?;
        let output = format!("{}.swf", mod_conf.name.to_case(Case::Snake));
        asconfigc.compiler_option.output = PathBuf::from("..")
            .join(&self.build_path)
            .join("res")
            .join("gui")
            .join("flash")
            .join(output)
            .to_string()?;
        asconfigc.write_json_to_file(&asconfigc_json_path)?;

        Ok(())
    }

    fn prompt_main_class(&self, default: &str) -> Result<String> {
        let value =
            inquire::Text::new("What's the main ui class path (from ui/src)")
                .with_default(default)
                .with_placeholder(default)
                .prompt()?;

        Ok(value)
    }

    fn make_archive(&self) -> Result<PathBuf> {
        let archive_file = self.target_path.join("result.wotmod");
        let compression_options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored);

        zip_create_from_directory_with_options(
            &archive_file,
            &self.build_path,
            |_| compression_options,
        )?;

        Ok(archive_file)
    }

    pub fn build(&self) -> Result<()> {
        self.throw_if_isn_t_mod_folder()?;

        self.clean_target_directory()?;

        self.build_python_src()?;
        self.copy_meta_file()?;

        self.build_flash_src()?;

        let archive_path = self.make_archive()?;
        let absolute_build_path = convert_to_absolute_path(&archive_path)?;
        println!("Build finished: {}", absolute_build_path);

        Ok(())
    }

    fn throw_if_isn_t_mod_folder(&self) -> Result<()> {
        let meta_path = self.mod_path.join("mod.json");

        if meta_path.exists() == false {
            let absolute_mod_folder_path =
                self.mod_path.canonicalize().map_err(|_| {
                    Error::PathError(
                        "failed to put in canonical form".to_string(),
                    )
                })?;

            return Err(Error::BadModFolderError(absolute_mod_folder_path));
        };

        Ok(())
    }
}
