mod constants;
mod environment;
mod fonts;
mod tailwind;
#[macro_use]
mod html;
mod assets;
mod calendar;
mod graphic_assets;
mod markdown;
mod mobs;
mod pages;
mod style;

use environment::OUTPUT_DIR;
use futures::{stream, StreamExt};
use ssg::generate_static_site;

#[tokio::main]
async fn main() {
    let assets = assets::get().await;
    let generated =
        stream::iter(generate_static_site(OUTPUT_DIR.parse().unwrap(), assets).unwrap())
            .map(|(path, source)| (path, tokio::spawn(source)))
            .for_each_concurrent(usize::MAX, |(path, join_handle)| async move {
                println!("generating: {path:?}");
                join_handle
                    .await
                    .unwrap()
                    .unwrap_or_else(|error| panic!("{path:?}: {error:?}"));
            });

    tokio::join!(generated);
    tailwind::execute().await;
}
