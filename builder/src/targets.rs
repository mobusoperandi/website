use camino::Utf8Path;
use ssg_child::sources::bytes_with_file_spec_safety::{TargetNotFoundError, Targets};

use crate::path::PathBuf;

pub(crate) trait TargetsExt {
    fn path_of(&self, path: impl AsRef<Utf8Path>) -> Result<PathBuf, TargetNotFoundError>;
    fn current_path(&self) -> PathBuf;
}

impl TargetsExt for Targets {
    fn path_of(&self, path: impl AsRef<Utf8Path>) -> Result<PathBuf, TargetNotFoundError> {
        self.path_of_(path).map(|path| path.into())
    }

    fn current_path(&self) -> PathBuf {
        self.current_path_().into()
    }
}
