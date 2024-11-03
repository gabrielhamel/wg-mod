use clap::builder::Str;

pub mod copy_directory;
pub mod downloader;
pub mod file_template;
pub mod pattern_validator;

pub mod convert_to_absolute_path;
pub mod convert_pathbuf_to_string;
pub mod command;

#[derive(Clone)]
pub struct Env {
    pub key: String,
    pub value: String,
}