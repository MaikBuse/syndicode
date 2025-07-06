use crate::application::ports::downloader::{BackupDownloader, DownloadError, DownloadResult};
use futures_util::TryStreamExt;
use reqwest::Client;
use tokio::io::AsyncRead;
use tokio_util::io::StreamReader;
use tonic::async_trait;

pub struct HttpBackupDownloader {
    client: Client,
}

impl HttpBackupDownloader {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl BackupDownloader for HttpBackupDownloader {
    async fn download(&self, source: String) -> DownloadResult<Box<dyn AsyncRead + Unpin + Send>> {
        let response = self.client.get(source.as_str()).send().await?;

        if !response.status().is_success() {
            return Err(DownloadError::DownloadStatus(response.status()));
        }

        let byte_stream = response.bytes_stream().map_err(std::io::Error::other);

        let stream_reader = StreamReader::new(byte_stream);

        Ok(Box::new(stream_reader))
    }
}
