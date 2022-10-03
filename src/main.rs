mod environment;
mod fonts;
mod markdown;
mod mobs;
mod page;
mod sections;
use crate::{mobs::read_mob, sections::sections};
use environment::OUTPUT_DIR;
use futures::{future::BoxFuture, stream, StreamExt};
use futures_util::FutureExt;
use std::{collections::BTreeMap, path::PathBuf};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;
use url::Url;

#[tokio::main]
async fn main() {
    //console_subscriber::init();
    let fonts = fonts::ssg_inputs().map(|(path, source)| (path, source.boxed()));
    let mobs = ReadDirStream::new(fs::read_dir("mobs").await.unwrap())
        .then(read_mob)
        .collect::<Vec<_>>()
        .await;
    let mob_pages = mobs
        .iter()
        .map(mobs::page)
        .map(|(path, source)| (path, source.boxed()))
        .collect::<Vec<_>>();
    let events = mobs.into_iter().fold(Vec::new(), mobs::events);
    let index_page = {
        let (path, source) = page::index(events);
        (path, source.boxed())
    };
    let favicon = (
        PathBuf::from("favicon.ico"),
        async { ssg::Source::Bytes(vec![]) }.boxed(),
    );
    let fullcalendar_css = (
        PathBuf::from("fullcalendar.css"),
        async {
            ssg::Source::Http(
                Url::parse("https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.css")
                    .unwrap(),
            )
        }
        .boxed(),
    );
    let fullcalendar_js = (
        PathBuf::from("fullcalendar.js"),
        async {
            ssg::Source::Http(
                Url::parse("https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.js").unwrap(),
            )
        }
        .boxed(),
    );
    let files: BTreeMap<PathBuf, BoxFuture<ssg::Source>> =
        [favicon, index_page, fullcalendar_css, fullcalendar_js]
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
