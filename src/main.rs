mod environment;
mod fonts;
mod markdown;
mod mobs;
mod page;
mod sections;
use crate::{mobs::read_mob, sections::sections};
use environment::OUTPUT_DIR;
use futures::{stream, StreamExt};
use futures_util::FutureExt;
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
    let favicon = ssg::Asset {
        source: async { ssg::Source::Bytes(vec![]) }.boxed(),
        target: PathBuf::from("favicon.ico"),
    };
    let fullcalendar_css = ssg::Asset {
        target: PathBuf::from("fullcalendar.css"),
        source: async {
            ssg::Source::Http(
                Url::parse("https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.css")
                    .unwrap(),
            )
        }
        .boxed(),
    };
    let fullcalendar_js = ssg::Asset {
        target: PathBuf::from("fullcalendar.js"),
        source: async {
            ssg::Source::Http(
                Url::parse("https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.js").unwrap(),
            )
        }
        .boxed(),
    };
    let files: BTreeSet<ssg::Asset> = [favicon, index_page, fullcalendar_css, fullcalendar_js]
        .into_iter()
        .chain(fonts)
        .chain(mob_pages)
        .collect();
    // TODO exit code
    let generated = stream::iter(ssg::generate_static_site(
        OUTPUT_DIR.parse().unwrap(),
        files,
    ))
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
