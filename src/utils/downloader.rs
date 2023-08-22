use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
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

// https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
pub async fn download_file(url: &str, path: &str) -> Result<(), Error> {
    let res = reqwest::get(url).await?;
    let total_size = res.content_length().ok_or(Error::ContentLengthError)?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/green}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("█▒▒"));
    pb.set_message(format!("Downloading {}", url));

    let mut file = File::create(path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {}", url));
    Ok(())
}
