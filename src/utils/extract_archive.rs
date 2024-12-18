use crate::builder::flash_lib::GameFlashLib;
use crate::utils::copy_directory::copy_directory;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use tempfile::tempdir;
use zip::result::ZipError;
use zip::write::{ExtendedFileOptions, FileOptions};
use zip::{CompressionMethod, ZipArchive, ZipWriter};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Archive does not exist: {0}")]
    ArchiveError(String),
    #[error("Failed to extract archive: {0}")]
    UnzipError(#[from] ZipError),
}

pub fn extract_archive(
    archive_path: &PathBuf, destination: &PathBuf,
) -> Result<(), Error> {
    let file = File::open(&archive_path)
        .map_err(|e| Error::ArchiveError(e.to_string()))?;

    let mut archive = ZipArchive::new(file)?;
    archive.extract(destination)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn extract_archive_test() {
        let tmp_dir = tempdir().unwrap();

        mock_archive(
            &tmp_dir.path().to_path_buf(),
            "build_test.zip".to_string(),
        );

        extract_archive(
            &tmp_dir.path().to_path_buf().join("build_test.zip"),
            &tmp_dir.path().to_path_buf(),
        )
        .unwrap();

        assert!(tmp_dir.path().join("build_test.txt").exists());
        tmp_dir.close().unwrap();
    }
}

fn mock_archive(path: &PathBuf, archive_name: String) {
    let file = File::create(path.join(archive_name)).unwrap();
    let writer = BufWriter::new(file);
    let mut zip = ZipWriter::new(writer);
    let options: FileOptions<'_, ExtendedFileOptions> =
        FileOptions::default().compression_method(CompressionMethod::Deflated);

    zip.start_file("build_test.txt", options).unwrap();
    zip.write_all(b"Hello, world!").unwrap();

    zip.finish().unwrap();
}
