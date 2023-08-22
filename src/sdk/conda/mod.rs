use crate::utils::downloader::download_file;

pub async fn download() {
    download_file(
        "https://repo.anaconda.com/miniconda/Miniconda3-latest-MacOSX-arm64.sh",
        "Miniconda3-latest-MacOSX-arm64.sh",
    )
    .await
    .unwrap();
}
