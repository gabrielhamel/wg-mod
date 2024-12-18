pub mod command;
pub mod convert_pathbuf_to_string;
pub mod convert_to_absolute_path;
pub mod copy_directory;
pub mod downloader;
pub mod extract_archive;
pub mod file_template;
pub mod pattern_validator;

#[derive(Clone, Debug)]
pub struct Env {
    pub key: String,
    pub value: String,
}
