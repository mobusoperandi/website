#![deny(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

#[macro_use]
mod html;

mod components;
mod constants;
mod expected_targets;
mod file_specs;
mod fonts;
mod google_font;
mod graphic_file_specs;
mod markdown;
mod mob;
mod pages;
mod relative_path;
mod style;
mod syn_helpers;
mod tailwind;
mod url;

use builder::OUTPUT_DIR;
use ssg_child::generate_static_site;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let file_specs = file_specs::get()?;
    let mut generation_task = generate_static_site(OUTPUT_DIR.clone(), file_specs);

    generation_task.on_target_result(|progress_report| {
        eprintln!("{progress_report:?}");
    });

    generation_task.await?;

    tailwind::execute().await?;

    Ok(())
}
