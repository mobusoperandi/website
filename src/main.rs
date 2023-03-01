#[macro_use]
mod html;

mod assets;
mod components;
mod constants;
mod fonts;
mod graphic_assets;
mod markdown;
mod mobs;
mod pages;
mod style;
mod tailwind;
mod url;

use std::thread;

use clap::{Parser, ValueEnum};
use futures::{stream, StreamExt};
use ssg::generate_static_site;
use tokio::process::Command;

use crate::constants::OUTPUT_DIR;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(value_enum, default_value_t = Mode::Build)]
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// build the website into the output directory and exit
    Build,
    /// watch for changes and rebuild the website
    /// and start a development web server
    Dev,
    /// print the output directory path
    PrintOutputDir,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Build => build().await,
        Mode::Dev => dev(),
        Mode::PrintOutputDir => print!("{OUTPUT_DIR}"),
    }
}

async fn build() {
    let assets = assets::get().await;
    stream::iter(generate_static_site(OUTPUT_DIR.parse().unwrap(), assets).unwrap())
        .map(|(path, source)| (path, tokio::spawn(source)))
        .for_each_concurrent(usize::MAX, |(path, join_handle)| async move {
            println!("generating: {path:?}");
            join_handle
                .await
                .unwrap()
                .unwrap_or_else(|error| panic!("{path:?}: {error:?}"));
        })
        .await;
    tailwind::execute().await;
}

fn dev() {
    watch_for_changes_and_rebuild();
    start_development_web_server();
    thread::park();
}

fn start_development_web_server() {
    Command::new("cargo")
        .args(["bin", "live-server", "--host", "localhost", OUTPUT_DIR])
        .spawn()
        .unwrap();
}

fn watch_for_changes_and_rebuild() {
    Command::new("cargo")
        .args(["bin", "cargo-watch", "--exec", "run -- build"])
        .spawn()
        .unwrap();
}
