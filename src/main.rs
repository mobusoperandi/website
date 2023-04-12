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

use std::{env::current_dir, path::PathBuf};

use ::url::Url;
use anyhow::anyhow;
use clap::{Parser, Subcommand};
use colored::Colorize;
use futures::{stream, StreamExt};
use ssg::{generate_static_site, FileGenerationError};
use tokio::process::Command;

use crate::constants::{LOCALHOST, LOCAL_DEV_PORT, OUTPUT_DIR};

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
        Some(Mode::Dev { open }) => dev(open).await?,
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

async fn dev(launch_browser: bool) -> anyhow::Result<()> {
    tokio::select! {
        result = watch_for_changes_and_rebuild() => { result?; },
        result = start_development_web_server(launch_browser) => { result?; },
    };

    Ok(())
}

async fn start_development_web_server(launch_browser: bool) -> Result<(), std::io::Error> {
    let url = Url::parse(&format!("http://{LOCALHOST}:{}", *LOCAL_DEV_PORT)).unwrap();
    let message = format!("\nServer started at {url}\n").blue();
    println!("{message}");

    if launch_browser {
        open::that(url.as_str())?;
    }

    live_server::listen(
        LOCALHOST,
        *LOCAL_DEV_PORT,
        [current_dir()?, OUTPUT_DIR.into()]
            .into_iter()
            .collect::<PathBuf>(),
    )
    .await
}

async fn watch_for_changes_and_rebuild() -> anyhow::Result<std::process::ExitStatus> {
    let mut child = Command::new("cargo")
        .args(["bin", "cargo-watch", "--exec", "run -- build"])
        .spawn()?;

    Ok(child.wait().await?)
}
