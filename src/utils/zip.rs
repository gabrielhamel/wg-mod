use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Component;
use std::path::PathBuf;
use zip::result::ZipResult;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};
use zip_extensions::zip_extract;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to create an zip archive: {0}")]
    ArchiveError(String),

    #[error("Unable to extract the archive: {0}")]
    ExtractError(String),
}

fn make_relative_path(root: &PathBuf, current: &PathBuf) -> PathBuf {
    let mut result = PathBuf::new();
    let root_components = root.components().collect::<Vec<Component>>();
    let current_components = current.components().collect::<Vec<_>>();
    for i in 0..current_components.len() {
        let current_path_component: Component = current_components[i];
        if i < root_components.len() {
            let other: Component = root_components[i];
            if other != current_path_component {
                break;
            }
        } else {
            result.push(current_path_component)
        }
    }
    result
}

fn path_as_string(path: &std::path::Path) -> String {
    let mut path_str = String::new();
    for component in path.components() {
        if let Component::Normal(os_str) = component {
            if !path_str.is_empty() {
                path_str.push('/');
            }
            path_str.push_str(&*os_str.to_string_lossy());
        }
    }
    path_str
}

trait ZipWriterExtensions {
    fn create_from_directory_with_options(
        self, directory: &PathBuf,
    ) -> ZipResult<()>;
}

impl<W: Write + io::Seek> ZipWriterExtensions for ZipWriter<W> {
    fn create_from_directory_with_options(
        mut self, directory: &PathBuf,
    ) -> ZipResult<()> {
        let file_options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored);

        let mut paths_queue: Vec<PathBuf> = vec![];
        paths_queue.push(directory.clone());

        let mut buffer = Vec::new();

        while let Some(next) = paths_queue.pop() {
            let directory_entry_iterator = std::fs::read_dir(next)?;

            for entry in directory_entry_iterator {
                let entry_path = entry?.path();
                let entry_metadata = std::fs::metadata(entry_path.clone())?;
                if entry_metadata.is_file() {
                    let mut f = File::open(&entry_path)?;
                    f.read_to_end(&mut buffer)?;
                    let relative_path =
                        make_relative_path(&directory, &entry_path);
                    self.start_file(
                        path_as_string(&relative_path),
                        file_options,
                    )?;
                    self.write_all(buffer.as_ref())?;
                    buffer.clear();
                } else if entry_metadata.is_dir() {
                    let relative_path =
                        make_relative_path(&directory, &entry_path);
                    self.add_directory(
                        path_as_string(&relative_path),
                        file_options,
                    )?;
                    paths_queue.push(entry_path.clone());
                }
            }
        }

        self.finish()?;
        Ok(())
    }
}

pub fn archive_directory(
    archive_file: &PathBuf, directory: &PathBuf,
) -> Result<(), Error> {
    let file = File::create(archive_file)
        .map_err(|error| Error::ArchiveError(error.to_string()))?;
    let zip_writer = ZipWriter::new(file);

    zip_writer
        .create_from_directory_with_options(directory)
        .map_err(|error| Error::ArchiveError(error.to_string()))
}

pub fn extract(
    archive_file: &PathBuf, directory: &PathBuf,
) -> Result<(), Error> {
    zip_extract(archive_file, directory)
        .map_err(|error| Error::ExtractError(error.to_string()))
}
