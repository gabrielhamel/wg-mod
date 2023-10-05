use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to GET")]
    FetchError(#[from] reqwest::Error),

    #[error("Error occured during file writting")]
    FileWriteError(#[from] std::io::Error),
}

pub async fn download_file(url: &str, path: &str) -> Result<(), Error> {
    let http_response = reqwest::get(url).await?;
    let mut file = File::create(path)?;
    let mut stream = http_response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
    }

    Ok(())
}
