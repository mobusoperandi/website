mod disk_caching_http_client;

use std::{
    collections::BTreeSet,
    fmt::Display,
    path::{Path, PathBuf},
};

use futures::{future::BoxFuture, Future, FutureExt};
use readext::ReadExt;
use reqwest::Url;
use thiserror::Error;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, thiserror::Error)]
#[error("Failed to generate {spec_target_path}: {source}")]
pub struct FileGenerationError {
    spec_target_path: PathBuf,
    source: FileGenerationErrorCause,
}

impl FileGenerationError {
    pub fn new(spec_target_path: PathBuf, source: FileGenerationErrorCause) -> Self {
        Self {
            spec_target_path,
            source,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FileGenerationErrorCause {
    #[error("user function error: {0}")]
    UserFnError(#[from] Box<dyn std::error::Error + Send>),
    #[error(transparent)]
    GoogleFontDownloadError(#[from] GoogleFontDownloadError),
    #[error(transparent)]
    RequestMiddlewareError(#[from] reqwest_middleware::Error),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
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
                let targets = Targets {
                    all: targets,
                    current: this_target.clone(),
                };

                let task = function(targets);

                task.await
                    .map_err(|error| FileGenerationError::new(this_target.clone(), error.into()))?
            }
            FileSource::GoogleFont(google_font) => google_font
                .download()
                .await
                .map_err(|error| FileGenerationError::new(this_target.clone(), error.into()))?,
            FileSource::Http(url) => {
                let client = disk_caching_http_client::create();
                client
                    .get(url.clone())
                    .send()
                    .await
                    .map_err(|error| FileGenerationError::new(this_target.clone(), error.into()))?
                    .bytes()
                    .await
                    .map_err(|error| FileGenerationError::new(this_target.clone(), error.into()))?
                    .to_vec()
            }
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

type FileSpecSafetyResult = Result<Vec<u8>, Box<dyn std::error::Error + Send>>;

pub enum FileSource {
    Static(&'static [u8]),
    BytesWithFileSpecSafety(
        Box<dyn Fn(Targets) -> BoxFuture<'static, FileSpecSafetyResult> + Send>,
    ),
    GoogleFont(GoogleFont),
    Http(Url),
}

#[derive(Debug, Clone)]
pub struct Targets {
    current: PathBuf,
    all: BTreeSet<PathBuf>,
}

#[derive(Debug, thiserror::Error)]
#[error("target not found: {target}")]
pub struct TargetNotFoundError {
    target: PathBuf,
}

impl TargetNotFoundError {
    pub fn new(target: PathBuf) -> Self {
        Self { target }
    }
}

impl Targets {
    pub fn path_of(&self, path: impl AsRef<Path>) -> Result<String, TargetNotFoundError> {
        let path = path.as_ref();

        assert!(path.is_absolute(), "path not absolute: {path:?}");

        self.all
            .contains(path)
            .then(|| {
                PathBuf::from_iter([PathBuf::from("/"), path.to_owned()])
                    .to_str()
                    .map(|path| path.to_owned())
            })
            .flatten()
            .map(|path| {
                if path == "/index.html" {
                    String::from("/")
                } else {
                    path
                }
            })
            .ok_or_else(|| TargetNotFoundError::new(path.to_owned()))
    }
    pub fn current_path(&self) -> String {
        self.path_of(&self.current).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GoogleFont {
    pub name: &'static str,
    pub version: u8,
    pub subset: &'static str,
    pub variant: &'static str,
}

#[derive(Debug, Error)]
pub enum GoogleFontDownloadError {
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error(transparent)]
    RequestMiddleware(#[from] reqwest_middleware::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl GoogleFont {
    pub(crate) async fn download(&self) -> Result<Vec<u8>, GoogleFontDownloadError> {
        // TODO: Consider reusing the client ->
        let url = Url::parse_with_params(
            &format!(
                "https://gwfh.mranftl.com/api/fonts/{}",
                self.name.to_lowercase(),
            ),
            [
                ("download", "zip"),
                ("subsets", self.subset),
                ("variants", self.variant),
            ],
        )?;

        let client = disk_caching_http_client::create();
        let archive = client.get(url.clone()).send().await?.bytes().await?;
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(archive))?;

        let mut font_file = archive.by_name(&format!(
            "{}-v{}-{}-{}.woff2",
            self.name.to_lowercase(),
            self.version,
            self.subset,
            self.variant
        ))?;

        let font_contents = font_file.read_into_vec()?;

        Ok(font_contents)
    }
}

impl Display for GoogleFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
