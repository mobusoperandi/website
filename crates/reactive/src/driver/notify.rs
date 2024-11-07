use std::{marker::PhantomData, path::PathBuf};

use futures::{
    channel::mpsc,
    executor::block_on,
    future::{pending, LocalBoxFuture},
    stream::LocalBoxStream,
    FutureExt, SinkExt, StreamExt,
};
use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};

use super::Driver;

pub use notify::{Error, Event, EventKind, Result};

pub struct FsChangeDriver<T> {
    watcher: RecommendedWatcher,
    sender: mpsc::Sender<Result<Event>>,
    paths: Vec<PathBuf>,
    boo: PhantomData<fn(T) -> PathBuf>,
}

impl<T> Driver for FsChangeDriver<T>
where
    PathBuf: From<T>,
{
    type Args = Vec<T>;
    type ConstructionError = notify::Error;
    type Input = ();
    type Output = LocalBoxStream<'static, Result<Event>>;

    fn new(paths: Self::Args) -> Result<(Self, Self::Output)> {
        let (sender, receiver) = mpsc::channel::<Result<Event>>(1);

        let mut sender_clone = sender.clone();

        let watcher = recommended_watcher(move |result: Result<Event>| {
            block_on(sender_clone.send(result))
                .expect("this closure gets sent to a blocking context");
        })?;

        let fs_change_driver = Self {
            watcher,
            sender,
            paths: paths.into_iter().map(Into::into).collect(),
            boo: PhantomData,
        };

        Ok((fs_change_driver, receiver.boxed_local()))
    }

    fn init(mut self, _input: Self::Input) -> LocalBoxFuture<'static, ()> {
        for path in self.paths {
            if let Err(error) = self.watcher.watch(&path, RecursiveMode::Recursive) {
                block_on(self.sender.send(Err(error))).unwrap();
                return pending().boxed_local();
            };
        }

        async move {
            let _watcher = self.watcher;
            pending().await
        }
        .boxed_local()
    }
}
