use getset::Getters;
use relative_path::RelativePathBuf;

#[derive(Debug, thiserror::Error, Getters)]
#[error("Failed to generate {path}: {source}")]
pub struct FileError {
    #[getset(get = "pub(crate)")]
    path: RelativePathBuf,
    source: FileErrorCause,
}

impl FileError {
    pub(crate) fn new(path: RelativePathBuf, source: FileErrorCause) -> Self {
        Self { path, source }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum FileErrorCause {
    #[error(transparent)]
    Source(Box<dyn std::error::Error + Send>),
    #[error(transparent)]
    OutputIo(#[from] std::io::Error),
}
