use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The full URL of the .dump file to restore from.
    /// If provided, the application will perform the restore.
    #[arg(long)]
    pub restore: Option<String>,
}
