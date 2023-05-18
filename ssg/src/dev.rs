use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::{
    rebuild_and_run::{watch_for_changes_and_rebuild, WatchError},
    server::start_development_web_server,
};

#[derive(Debug, Error)]
pub enum DevError {
    #[error(transparent)]
    Watch(WatchError),
    #[error(transparent)]
    Io(std::io::Error),
}

pub async fn dev<O: AsRef<Path>>(launch_browser: bool, output_dir: O) -> DevError {
    tokio::select! {
        error = watch_for_changes_and_rebuild() => { DevError::Watch(error) },
        error = start_development_web_server(launch_browser, PathBuf::from(output_dir.as_ref())) => { DevError::Io(error) },
    }
}
