mod environment;
mod fonts;
mod markdown;
mod mobs;
mod page;
mod sections;
use crate::{mobs::read_mob, sections::sections};
use environment::OUTPUT_DIR;
use futures::{stream, StreamExt};
use ssg::{generate_static_site, Asset, Source};
use std::{collections::BTreeSet, path::PathBuf};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;
use url::Url;

#[tokio::main]
async fn main() {
    //console_subscriber::init();
    let fonts = fonts::assets();
    let mobs = ReadDirStream::new(fs::read_dir("mobs").await.unwrap())
        .then(read_mob)
        .collect::<Vec<_>>()
        .await;
    let mob_pages = mobs.iter().map(mobs::page).collect::<Vec<_>>();
    let events = mobs.into_iter().fold(Vec::new(), mobs::events);
    let index_page = page::index(events);
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
    let files: BTreeSet<Asset> = [favicon, index_page, fullcalendar_css, fullcalendar_js]
        .into_iter()
        .chain(fonts)
        .chain(mob_pages)
        .collect();
    // TODO exit code
    let generated = stream::iter(generate_static_site(OUTPUT_DIR.parse().unwrap(), files))
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
