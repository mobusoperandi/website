mod disk_caching_http_client;
mod file_spec;
pub mod sources;
pub mod target_error;
pub mod target_success;

use std::collections::BTreeSet;

use camino::Utf8PathBuf;
pub use file_spec::FileSpec;
use futures::{stream, Stream, StreamExt};
use relative_path::RelativePathBuf;
use sources::bytes_with_file_spec_safety::Targets;
use target_error::TargetError;
use target_success::TargetSuccess;

/// Panics on duplicate `FileSpec` targets
pub fn generate_static_site(
    output_dir: Utf8PathBuf,
    file_specs: impl IntoIterator<Item = FileSpec>,
) -> impl Stream<Item = Result<TargetSuccess, TargetError>> {
    let (paths, file_specs) = file_specs.into_iter().fold(
        (BTreeSet::<RelativePathBuf>::new(), Vec::<FileSpec>::new()),
        |(mut paths, mut file_specs), file_spec| {
            let newly_inserted = paths.insert(file_spec.target().clone());

            if !newly_inserted {
                panic!("Duplicate target: {}", file_spec.target());
            }

            file_specs.push(file_spec);

            (paths, file_specs)
        },
    );

    stream::iter(file_specs)
        .map(move |file_spec| file_spec.generate(paths.clone(), output_dir.clone()))
        .buffer_unordered(usize::MAX)
}
