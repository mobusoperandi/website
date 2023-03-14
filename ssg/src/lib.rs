mod disk_caching_http_client;
pub mod sources;

use std::{collections::BTreeSet, path::PathBuf};

use futures::{future::BoxFuture, Future, FutureExt};
use sources::FileSource;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, thiserror::Error)]
#[error("Failed to generate {spec_target_path}: {source}")]
pub struct FileGenerationError {
    spec_target_path: PathBuf,
    source: FileGenerationErrorCause,
}

impl FileGenerationError {
    fn new(spec_target_path: PathBuf, source: FileGenerationErrorCause) -> Self {
        Self {
            spec_target_path,
            source,
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum FileGenerationErrorCause {
    #[error("user function error: {0}")]
    UserFn(#[from] Box<dyn std::error::Error + Send>),
    #[error(transparent)]
    GoogleFontDownload(#[from] sources::google_font::DownloadError),
    #[error(transparent)]
    RequestMiddleware(#[from] reqwest_middleware::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Panics on duplicate `FileSpec` targets
pub fn generate_static_site(
    output_dir: PathBuf,
    file_specs: impl IntoIterator<Item = FileSpec>,
) -> impl Iterator<Item = impl Future<Output = Result<(), FileGenerationError>>> {
    let (paths, file_specs) = file_specs.into_iter().fold(
        (BTreeSet::<PathBuf>::new(), BTreeSet::<FileSpec>::new()),
        |(mut paths, mut file_specs), file_spec| {
            let newly_inserted = paths.insert(file_spec.target.clone());

            if !newly_inserted {
                panic!("Duplicate target: {}", file_spec.target.display());
            }

            file_specs.insert(file_spec);

            (paths, file_specs)
        },
    );

    file_specs
        .into_iter()
        .map(move |FileSpec { source, target }| {
            generate_file_from_spec(source, paths.clone(), target, output_dir.clone())
        })
}

fn generate_file_from_spec(
    source: FileSource,
    targets: BTreeSet<PathBuf>,
    this_target: PathBuf,
    output_dir: PathBuf,
) -> BoxFuture<'static, Result<(), FileGenerationError>> {
    async {
        let contents = match source {
            FileSource::Static(bytes) => bytes.to_vec(),
            FileSource::BytesWithFileSpecSafety(function) => {
                sources::bytes_with_file_spec_safety::obtain(function, targets, this_target.clone())
                    .await
                    .map_err(|error| FileGenerationError::new(this_target.clone(), error.into()))?
            }
            FileSource::GoogleFont(google_font) => google_font
                .download()
                .await
                .map_err(|error| FileGenerationError::new(this_target.clone(), error.into()))?,
            FileSource::Http(url) => sources::http::download(url)
                .await
                .map_err(|error| FileGenerationError::new(this_target.clone(), error.into()))?,
        };

        let this_target_relative = this_target.iter().skip(1).collect();
        let file_path = [output_dir, this_target_relative]
            .into_iter()
            .collect::<PathBuf>();

        fs::create_dir_all(file_path.parent().unwrap())
            .await
            .unwrap();

        let mut file_handle = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .await
            .map_err(|error| FileGenerationError::new(this_target.clone(), error.into()))?;

        file_handle
            .write_all(&contents)
            .await
            .map_err(|error| FileGenerationError::new(this_target, error.into()))?;

        Ok(())
    }
    .boxed()
}

pub struct FileSpec {
    pub(crate) source: FileSource,
    pub(crate) target: PathBuf,
}

impl FileSpec {
    pub fn new<T>(target: T, source: FileSource) -> Self
    where
        PathBuf: From<T>,
    {
        let target: PathBuf = target.into();

        assert!(target.is_absolute(), "path not absolute: {target:?}");

        Self { source, target }
    }
}

impl PartialEq for FileSpec {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
    }
}

impl Eq for FileSpec {}

impl PartialOrd for FileSpec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileSpec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.target.cmp(&other.target)
    }
}
