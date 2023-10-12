#![warn(clippy::all, clippy::pedantic)]

use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use once_cell::sync::Lazy;
use ssg_parent::{DevError, Parent};

pub static OUTPUT_DIR: Lazy<Utf8PathBuf> =
    Lazy::new(|| Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".vercel/output/static"));

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Mode {
    /// watch for changes and rebuild the website
    /// and start a development web server
    Dev {
        /// open website in a browser
        #[arg(short, long)]
        open: bool,
    },
    /// print the output directory path
    PrintOutputDir,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Dev { open: true }
    }
}

#[tokio::main]
async fn main() -> Result<(), DevError> {
    #[cfg(feature = "tokio_console")]
    console_subscriber::init();

    let cli = Cli::parse();

    let parent = Parent::new(OUTPUT_DIR.clone());

    match cli.mode.unwrap_or_default() {
        Mode::Dev { open } => return Err(parent.dev(open).await),
        Mode::PrintOutputDir => print!("{}", OUTPUT_DIR.as_os_str().to_str().unwrap()),
    }

    Ok(())
}
