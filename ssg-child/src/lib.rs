mod disk_caching_http_client;
pub mod sources;

use std::{collections::BTreeSet, path::PathBuf};

use futures::{future::BoxFuture, stream, FutureExt, Stream, StreamExt};
use sources::{bytes_with_file_spec_safety::Targets, FileSource};
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, thiserror::Error)]
#[error("Failed to generate {spec_target_path}: {source}")]
pub struct TargetError {
    spec_target_path: PathBuf,
    source: TargetErrorCause,
}

impl TargetError {
    fn new(spec_target_path: PathBuf, source: TargetErrorCause) -> Self {
        Self {
            spec_target_path,
            source,
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum TargetErrorCause {
    #[error(transparent)]
    Source(Box<dyn std::error::Error + Send>),
    #[error(transparent)]
    TargetIo(#[from] std::io::Error),
}

/// Panics on duplicate `FileSpec` targets
pub fn generate_static_site(
    output_dir: PathBuf,
    file_specs: impl IntoIterator<Item = FileSpec>,
) -> impl Stream<Item = Result<(), TargetError>> {
    let (paths, file_specs) = file_specs.into_iter().fold(
        (BTreeSet::<PathBuf>::new(), Vec::<FileSpec>::new()),
        |(mut paths, mut file_specs), file_spec| {
            let newly_inserted = paths.insert(file_spec.target.clone());

            if !newly_inserted {
                panic!("Duplicate target: {}", file_spec.target.display());
            }

            file_specs.push(file_spec);

            (paths, file_specs)
        },
    );

    stream::iter(file_specs)
        .map(move |FileSpec { source, target }| {
            generate_file_from_spec(source, paths.clone(), target, output_dir.clone())
        })
        .buffer_unordered(usize::MAX)
}

fn generate_file_from_spec(
    source: Box<dyn FileSource + Send>,
    targets: BTreeSet<PathBuf>,
    this_target: PathBuf,
    output_dir: PathBuf,
) -> BoxFuture<'static, Result<(), TargetError>> {
    async move {
        let targets = Targets::new(this_target.clone(), targets);
        let task = source.obtain_content(targets);
        let this_target_relative = this_target.iter().skip(1).collect();

        let file_path = [output_dir, this_target_relative]
            .into_iter()
            .collect::<PathBuf>();

        fs::create_dir_all(file_path.parent().unwrap())
            .await
            .map_err(|error| {
                TargetError::new(this_target.clone(), TargetErrorCause::TargetIo(error))
            })?;

        let contents = task.await.map_err(|error| {
            TargetError::new(this_target.clone(), TargetErrorCause::Source(error))
        })?;

        let mut file_handle = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .await
            .map_err(|error| {
                TargetError::new(this_target.clone(), TargetErrorCause::TargetIo(error))
            })?;

        file_handle
            .write_all(&contents)
            .await
            .map_err(|error| TargetError::new(this_target, TargetErrorCause::TargetIo(error)))?;

        Ok(())
    }
    .boxed()
}

pub struct FileSpec {
    source: Box<dyn FileSource + Send>,
    target: PathBuf,
}

impl FileSpec {
    pub fn new<T>(target: T, source: impl FileSource + 'static + Send) -> Self
    where
        PathBuf: From<T>,
    {
        let target: PathBuf = target.into();

        assert!(target.is_absolute(), "path not absolute: {target:?}");

        Self {
            source: Box::new(source),
            target,
        }
    }
}
