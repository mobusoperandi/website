#![deny(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

mod disk_caching_http_client;
mod file_spec;
pub mod final_error;
pub mod generation_task;
pub mod sources;
pub mod target_error;
pub mod target_success;

use camino::Utf8PathBuf;
pub use file_spec::FileSpec;
use futures::{stream, StreamExt};
use generation_task::GenerationTask;

use target_error::TargetError;

pub fn generate_static_site(
    output_dir: Utf8PathBuf,
    file_specs: impl IntoIterator<Item = FileSpec> + 'static,
) -> GenerationTask {
    let tasks = stream::iter(file_specs)
        .map(move |file_spec| file_spec.generate(output_dir.clone()))
        .buffer_unordered(usize::MAX);

    GenerationTask::new(tasks)
}
