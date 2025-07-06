use tokio::io::AsyncRead;
use tonic::async_trait;

#[derive(thiserror::Error, Debug)]
pub enum DownloadError {
    #[error("Download failed: {0}")]
    Download(#[from] reqwest::Error),

    #[error("HTTP request failed with status: {0}")]
    DownloadStatus(reqwest::StatusCode),
}

pub type DownloadResult<T> = Result<T, DownloadError>;

/// Returns a readable stream of the backup data.
#[async_trait]
pub trait BackupDownloader: Send + Sync {
    async fn download(&self, source: String) -> DownloadResult<Box<dyn AsyncRead + Unpin + Send>>;
}
