#![warn(clippy::all, clippy::pedantic)]

mod disk_caching_http_client;
pub mod file_error;
pub mod file_msg;
mod file_spec;
pub mod file_success;
pub mod final_error;
pub mod generation_task;
pub mod sources;

use camino::Utf8PathBuf;
use file_msg::FileMsg;
pub use file_spec::FileSpec;
use futures::{stream, StreamExt};
use generation_task::GenerationTask;

use file_error::FileError;
use ipc_channel::ipc::IpcSender;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(Box<dyn std::error::Error + Send + Sync>);

impl Error {
    fn new(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self(Box::new(error))
    }
}

pub fn generate_static_site(
    file_specs: impl IntoIterator<Item = FileSpec> + 'static,
) -> Result<GenerationTask, Error> {
    let key = std::env::var(FileMsg::CHANNEL_ENV_VAR).map_err(Error::new)?;

    let sender = IpcSender::<FileMsg>::connect(key).map_err(Error::new)?;

    let tasks = stream::iter(file_specs)
        .map(move |file_spec| file_spec.generate(sender.clone()))
        .buffer_unordered(usize::MAX);

    Ok(GenerationTask::new(tasks))
}
