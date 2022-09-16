mod environment;
mod fonts;
mod markdown;
mod mobs;
mod page;
mod sections;
mod ssg;
use crate::{mobs::read_mob, sections::sections};
use environment::OUTPUT_DIR;
use futures::{future::BoxFuture, Stream, StreamExt};
use futures_util::FutureExt;
use mobs::Mob;
use std::{future::ready, path::PathBuf, pin::Pin};
use tokio::{fs, sync::broadcast};
use tokio_stream::wrappers::{BroadcastStream, ReadDirStream};

#[tokio::main]
async fn main() {
    //console_subscriber::init();
    let fonts = fonts::ssg_inputs();
    let mobs = ReadDirStream::new(fs::read_dir("mobs").await.unwrap()).then(read_mob);
    let (sender, receiver) = broadcast::channel::<Mob>(128);
    let mob_pages =
        BroadcastStream::new(sender.subscribe()).map(|mob_msg| mobs::page(mob_msg.unwrap()));
    let index_page = BroadcastStream::new(receiver)
        .map(|mob_msg| mob_msg.unwrap())
        .fold(Vec::new(), mobs::events)
        .map(page::index);
    let favicon = ssg::Input {
        target_path: PathBuf::from(OUTPUT_DIR).join("favicon.ico"),
        source: ssg::Source::Bytes(vec![]),
    };
    let files: [Pin<Box<dyn Stream<Item = BoxFuture<ssg::Input>>>>; 4] = [
        Box::pin(futures::stream::once(ready(ready(favicon).boxed()))),
        Box::pin(futures::stream::iter(fonts).map(|font| async { font }.boxed())),
        Box::pin(futures::stream::once(async { index_page.boxed() })),
        Box::pin(mob_pages.map(|mob_file| async { mob_file }.boxed())),
    ];
    let files = futures::stream::select_all(files);
    // TODO exit code
    let generated = ssg::generate_static_site(files)
        .map(tokio::spawn)
        .for_each_concurrent(usize::MAX, |join_handle| async {
            match join_handle.await.unwrap() {
                Ok(file) => {
                    println!("generated: {:?}", file.target_path);
                }
                // TODO know which file errored
                Err(error) => {
                    println!("error: {:?}", error);
                }
            }
        });
    let mobs_transmitted = async {
        mobs.for_each_concurrent(usize::MAX, |mob| async {
            sender.send(mob).unwrap();
        })
        .await;
        drop(sender);
    };
    tokio::join!(generated, mobs_transmitted);
}
