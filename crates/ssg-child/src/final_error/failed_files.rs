use std::collections::BTreeSet;

use relative_path::RelativePathBuf;

#[derive(Debug, Clone, thiserror::Error)]
#[error("failed files: {0:?}")]
pub(super) struct FailedFiles(BTreeSet<RelativePathBuf>);

impl FailedFiles {
    pub(crate) fn new(failed_files: BTreeSet<RelativePathBuf>) -> Self {
        Self(failed_files)
    }
}
