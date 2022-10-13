mod environment;
mod fonts;
mod markdown;
mod mobs;
mod pages;
use environment::OUTPUT_DIR;
use futures::{stream, StreamExt};
use ssg::{generate_static_site, Asset, Source};
use std::{collections::BTreeSet, path::PathBuf};
use url::Url;

#[tokio::main]
async fn main() {
    //console_subscriber::init();
    let fonts = fonts::assets();
    let mobs = mobs::read_all_mobs().await;
    let mob_pages = mobs.iter().map(mobs::page).collect::<Vec<_>>();
    let pages = pages::all().await;
    let favicon = Asset::new(PathBuf::from("favicon.ico"), async {
        Source::Bytes(vec![])
    });
    let fullcalendar_css = Asset::new(PathBuf::from("fullcalendar.css"), async {
        Source::Http(
            Url::parse("https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.css").unwrap(),
        )
    });
    let fullcalendar_js = Asset::new(PathBuf::from("fullcalendar.js"), async {
        Source::Http(
            Url::parse("https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.js").unwrap(),
        )
    });
    let files: BTreeSet<Asset> = [favicon, fullcalendar_css, fullcalendar_js]
        .into_iter()
        .chain(fonts)
        .chain(pages)
        .chain(mob_pages)
        .collect();
    // TODO exit code
    let generated = stream::iter(generate_static_site(OUTPUT_DIR.parse().unwrap(), files).unwrap())
        .map(|(path, source)| (path, tokio::spawn(source)))
        .for_each_concurrent(usize::MAX, |(path, join_handle)| async move {
            match join_handle.await.unwrap() {
                Ok(()) => {
                    println!("generated: {:?}", path);
                }
                // TODO know which file errored
                Err(error) => {
                    println!("error: {:?}", error);
                }
            }
        });
    tokio::join!(generated);
}
