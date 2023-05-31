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

use anyhow::bail;
use builder::OUTPUT_DIR;
use ssg_child::generate_static_site;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_specs = file_specs::get().await;

    if let Err(err) = generate_static_site(OUTPUT_DIR.clone(), file_specs).await {
        bail!("{err}")
    };

    tailwind::execute().await?;
    Ok(())
}
