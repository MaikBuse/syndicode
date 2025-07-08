use super::postgres::build_postgres_db_url;
use crate::application::ports::restorer::{DatabaseRestorer, RestoreError, RestoreResult};
use crate::config::ServerConfig;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tonic::async_trait;

pub struct PgRestoreExecutor {
    db_url: String,
}

impl PgRestoreExecutor {
    pub fn new(config: Arc<ServerConfig>) -> Self {
        let db_url = build_postgres_db_url(config);

        Self { db_url }
    }
}

#[async_trait]
impl DatabaseRestorer for PgRestoreExecutor {
    async fn restore(
        &self,
        mut data_stream: Box<dyn AsyncRead + Unpin + Send>,
    ) -> RestoreResult<()> {
        let mut command = Command::new("pg_restore");
        command
            .arg("--verbose")
            .arg("--clean")
            .arg("--if-exists")
            .arg("--no-owner")
            .arg("--no-acl")
            .arg("--dbname")
            .arg(self.db_url.as_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command.spawn().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                RestoreError::CommandNotFound("pg_restore".to_string())
            } else {
                RestoreError::Io(e)
            }
        })?;

        let mut stdin = child
            .stdin
            .take()
            .expect("Failed to open stdin for pg_restore");
        let mut stdout = child
            .stdout
            .take()
            .expect("Failed to open stdout for pg_restore");
        let mut stderr = child
            .stderr
            .take()
            .expect("Failed to open stderr for pg_restore");

        // Spawn tasks to read stdout and stderr concurrently so we don't miss anything
        let stdout_handle = tokio::spawn(async move {
            let mut buf = String::new();
            let _ = stdout.read_to_string(&mut buf).await;
            buf
        });

        let stderr_handle = tokio::spawn(async move {
            let mut buf = String::new();
            let _ = stderr.read_to_string(&mut buf).await;
            buf
        });

        // Copy data from the download stream to the process's stdin
        let copy_result = tokio::io::copy(&mut data_stream, &mut stdin).await;

        // Ensure stdin is closed to signal EOF to pg_restore, just in case `copy` didn't.
        let _ = stdin.shutdown().await;
        drop(stdin);

        if let Err(e) = copy_result {
            // Wait for the spawned tasks to finish reading stdout/stderr
            let collected_stderr = stderr_handle
                .await
                .unwrap_or_else(|_| "Failed to collect stderr".to_string());
            let collected_stdout = stdout_handle
                .await
                .unwrap_or_else(|_| "Failed to collect stdout".to_string());

            // Log the captured output with tracing::error!
            tracing::error!(
                error_message = %e,
                pg_restore_stdout = %collected_stdout,
                pg_restore_stderr = %collected_stderr,
                "Failed to copy data to pg_restore stdin. The process likely exited early."
            );

            // Return the original I/O error to maintain existing error flow
            return Err(RestoreError::Io(e));
        }

        let status = child.wait().await?;

        // Collect the final output from the handles
        let final_stdout = stdout_handle.await.unwrap_or_default();
        let final_stderr = stderr_handle.await.unwrap_or_default();

        if status.success() {
            if !final_stderr.is_empty() {
                // Log verbose output from stderr even on success
                tracing::info!(pg_restore_stderr = %final_stderr, "pg_restore completed with verbose messages.");
            }
            Ok(())
        } else {
            tracing::error!(
                exit_code = ?status.code(),
                pg_restore_stdout = %final_stdout,
                pg_restore_stderr = %final_stderr,
                "pg_restore command failed."
            );
            Err(RestoreError::CommandFailed {
                stdout: final_stdout,
                stderr: final_stderr,
            })
        }
    }
}
