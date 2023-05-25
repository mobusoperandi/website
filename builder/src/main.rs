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

use anyhow::anyhow;
use builder::OUTPUT_DIR;
use futures::{stream, StreamExt};
use ssg_child::{generate_static_site, FileGenerationError};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_specs = file_specs::get().await;

    stream::iter(generate_static_site(OUTPUT_DIR.clone(), file_specs))
        .buffer_unordered(usize::MAX)
        .collect::<Vec<Result<(), FileGenerationError>>>()
        .await
        .into_iter()
        .collect::<Result<(), _>>()
        .map_err(|error| anyhow!("{error}"))?;

    tailwind::execute().await?;
    Ok(())
}
