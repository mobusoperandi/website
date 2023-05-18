#[macro_use]
mod html;

mod components;
mod constants;
mod file_specs;
mod fonts;
mod graphic_file_specs;
mod markdown;
mod mob;
mod pages;
mod style;
mod syn_helpers;
mod tailwind;
mod url;

use anyhow::{anyhow, bail};
use clap::{Parser, Subcommand};
use futures::{stream, StreamExt};
use ssg::{dev, generate_static_site, FileGenerationError};

use crate::constants::OUTPUT_DIR;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Mode {
    /// build the website into the output directory and exit
    Build,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "tokio_console")]
    console_subscriber::init();

    let cli = Cli::parse();

    match cli.mode {
        None | Some(Mode::Build) => build().await?,
        Some(Mode::Dev { open }) => bail!(dev(open, OUTPUT_DIR).await),
        Some(Mode::PrintOutputDir) => print!("{OUTPUT_DIR}"),
    }

    Ok(())
}

async fn build() -> anyhow::Result<()> {
    let file_specs = file_specs::get().await;

    stream::iter(generate_static_site(OUTPUT_DIR.parse()?, file_specs))
        .buffer_unordered(usize::MAX)
        .collect::<Vec<Result<(), FileGenerationError>>>()
        .await
        .into_iter()
        .collect::<Result<(), _>>()
        .map_err(|error| anyhow!("{error}"))?;

    tailwind::execute().await?;
    Ok(())
}
