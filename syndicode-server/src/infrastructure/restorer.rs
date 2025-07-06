use std::process::Stdio;
use std::sync::Arc;
use tokio::io::AsyncRead;
use tokio::process::Command;
use tonic::async_trait;

use crate::application::ports::restorer::{DatabaseRestorer, RestoreError, RestoreResult};
use crate::config::ServerConfig;

use super::postgres::build_postgres_db_url;

pub struct PgRestoreExecutor;

#[async_trait]
impl DatabaseRestorer for PgRestoreExecutor {
    async fn restore(
        &self,
        config: Arc<ServerConfig>,
        mut data_stream: Box<dyn AsyncRead + Unpin + Send>,
    ) -> RestoreResult<()> {
        let db_url = build_postgres_db_url(config);

        let mut command = Command::new("pg_restore");
        command
            .arg("--verbose")
            .arg("--clean")
            .arg("--if-exists")
            .arg("--no-owner")
            .arg("--no-acl")
            .arg("--dbname")
            .arg(db_url)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(RestoreError::CommandNotFound("pg_restore".to_string()));
            }
            Err(e) => return Err(RestoreError::Io(e)),
        };

        let mut stdin = child.stdin.take().expect("Failed to open stdin");

        // Asynchronously copy data from the download stream to the process stdin
        tokio::io::copy(&mut data_stream, &mut stdin).await?;
        drop(stdin); // Close stdin to signal end of data

        let output = child.wait_with_output().await?;

        if output.status.success() {
            Ok(())
        } else {
            Err(RestoreError::CommandFailed {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
    }
}
