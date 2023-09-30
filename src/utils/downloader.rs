use crate::utils::task_progress::TaskProgression;
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to GET")]
    FetchError(#[from] reqwest::Error),

    #[error("The server didn't send content length")]
    ContentLengthError,

    #[error("Failed to initialize progress bar")]
    TemplateError(#[from] indicatif::style::TemplateError),

    #[error("Error occured during file writting")]
    FileWriteError(#[from] std::io::Error),
}

pub async fn download_file<T: TaskProgression>(
    url: &str, path: &str, mut task_progression: T,
) -> Result<(), Error> {
    let http_response = reqwest::get(url).await?;
    let total_bytes_to_download = http_response
        .content_length()
        .ok_or(Error::ContentLengthError)?;
    let mut file = File::create(path)?;
    let mut bytes_downloaded: u64 = 0;
    let mut stream = http_response.bytes_stream();

    task_progression.start();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;

        bytes_downloaded += chunk.len() as u64;
        task_progression
            .progress(bytes_downloaded as f32 / total_bytes_to_download as f32);
    }

    task_progression.end();

    Ok(())
}
