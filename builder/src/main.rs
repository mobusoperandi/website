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

use builder::OUTPUT_DIR;
use ssg_child::generate_static_site;

use bool_ext::BoolExt;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let file_specs = file_specs::get().await;

    generate_static_site(OUTPUT_DIR.clone(), file_specs)
        .all(|progress_report| async move {
            eprintln!("{progress_report:?}");

            progress_report.is_ok()
        })
        .await
        .to_result()?;

    tailwind::execute().await.map_err(|_| ())?;

    Ok(())
}
