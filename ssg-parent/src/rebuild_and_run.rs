use std::path::PathBuf;

use async_fn_stream::fn_stream;
use futures::TryStreamExt;
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};
use thiserror::Error;
use tokio::{
    process::{Child, Command},
    sync::mpsc,
};

#[derive(Error, Debug)]
pub enum WatchError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Notify(#[from] notify::Error),
}

const BUILDER_CRATE_NAME: &str = "builder";

pub async fn watch_for_changes_and_rebuild() -> WatchError {
    let child = match cargo_run_builder() {
        Ok(child) => child,
        Err(error) => return error.into(),
    };

    let (sender, mut receiver) = mpsc::channel(1);

    let watcher = recommended_watcher(move |result: Result<Event, notify::Error>| {
        sender.blocking_send(result).unwrap();
    });

    let mut watcher = match watcher {
        Ok(watcher) => watcher,
        Err(error) => return error.into(),
    };

    if let Err(error) = watcher.watch(&PathBuf::from(BUILDER_CRATE_NAME), RecursiveMode::Recursive)
    {
        return error.into();
    }

    fn_stream(|emitter| async move {
        loop {
            let event = receiver.recv().await.unwrap();
            emitter.emit(event).await;
        }
    })
    .try_fold(child, |mut child, event: Event| async move {
        if let EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) = event.kind {
            child.kill().await?;
            Ok(cargo_run_builder()?)
        } else {
            Ok(child)
        }
    })
    .await
    .expect_err("should end only in the case of error")
    .into()
}

fn cargo_run_builder() -> Result<Child, std::io::Error> {
    Command::new("cargo")
        .args(["run", "--package", BUILDER_CRATE_NAME])
        .spawn()
}
