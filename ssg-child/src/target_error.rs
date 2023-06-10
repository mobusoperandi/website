use relative_path::RelativePathBuf;

#[derive(Debug, thiserror::Error)]
#[error("Failed to generate {spec_target_path}: {source}")]
pub struct TargetError {
    spec_target_path: RelativePathBuf,
    source: TargetErrorCause,
}

impl TargetError {
    pub(crate) fn new(spec_target_path: RelativePathBuf, source: TargetErrorCause) -> Self {
        Self {
            spec_target_path,
            source,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum TargetErrorCause {
    #[error(transparent)]
    Source(Box<dyn std::error::Error + Send>),
    #[error(transparent)]
    TargetIo(#[from] std::io::Error),
}
