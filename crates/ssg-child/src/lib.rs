#![warn(clippy::all, clippy::pedantic)]

mod disk_caching_http_client;
pub mod file_error;
mod file_spec;
pub mod file_success;
pub mod final_error;
pub mod generation_task;
pub mod sources;

use camino::Utf8PathBuf;
pub use file_spec::FileSpec;
use futures::{stream, StreamExt};
use generation_task::GenerationTask;

use file_error::FileError;

pub fn generate_static_site(
    output_dir: Utf8PathBuf,
    file_specs: impl IntoIterator<Item = FileSpec> + 'static,
) -> GenerationTask {
    let tasks = stream::iter(file_specs)
        .map(move |file_spec| file_spec.generate(output_dir.clone()))
        .buffer_unordered(usize::MAX);

    GenerationTask::new(tasks)
}
