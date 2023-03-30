#[macro_use]
mod html;

mod components;
mod constants;
mod file_specs;
mod fonts;
mod graphic_file_specs;
mod markdown;
mod mobs;
mod pages;
mod style;
mod syn_helpers;
mod tailwind;
mod url;

use std::{env::current_dir, path::PathBuf};

use ::url::Url;
use clap::{Parser, Subcommand};
use colored::Colorize;
use futures::{stream, StreamExt};
use ssg::generate_static_site;
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
async fn main() {
    #[cfg(feature = "tokio_console")]
    console_subscriber::init();

    let cli = Cli::parse();

    match cli.mode {
        None | Some(Mode::Build) => build().await,
        Some(Mode::Dev { open }) => dev(open).await,
        Some(Mode::PrintOutputDir) => print!("{OUTPUT_DIR}"),
    }
}

async fn build() {
    let file_specs = file_specs::get().await;

    stream::iter(generate_static_site(
        OUTPUT_DIR.parse().unwrap(),
        file_specs,
    ))
    .for_each_concurrent(usize::MAX, |file_spec_task| async move {
        file_spec_task.await.unwrap();
    })
    .await;

    tailwind::execute().await;
}

async fn dev(launch_browser: bool) {
    tokio::select! {
        result = watch_for_changes_and_rebuild() => { result.unwrap(); },
        result = start_development_web_server(launch_browser) => { result.unwrap(); },
    };
}

async fn start_development_web_server(launch_browser: bool) -> Result<(), std::io::Error> {
    let url = Url::parse(&format!("http://{LOCALHOST}:{}", *LOCAL_DEV_PORT)).unwrap();
    let message = format!("\nServer started at {url}\n").blue();
    println!("{message}");

    if launch_browser {
        open::that(url.as_str()).unwrap();
    }

    live_server::listen(
        LOCALHOST,
        *LOCAL_DEV_PORT,
        [current_dir().unwrap(), OUTPUT_DIR.into()]
            .into_iter()
            .collect::<PathBuf>(),
    )
    .await
}

async fn watch_for_changes_and_rebuild() -> Result<std::process::ExitStatus, std::io::Error> {
    let mut child = Command::new("cargo")
        .args(["bin", "cargo-watch", "--exec", "run -- build"])
        .spawn()
        .unwrap();

    child.wait().await
}
