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
    let file_specs = file_specs::get().await;

    stream::iter(generate_static_site(
        OUTPUT_DIR.parse().unwrap(),
        file_specs,
    ))
    .map(tokio::spawn)
    .for_each_concurrent(usize::MAX, |join_handle| async move {
        join_handle.await.unwrap().unwrap();
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
