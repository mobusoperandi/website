use relative_path::RelativePathBuf;

use crate::sources::ExpectedFiles;

#[derive(Debug, Clone, getset::Getters)]
pub struct FileSuccess {
    #[getset(get = "pub(crate)")]
    path: RelativePathBuf,
    #[getset(get = "pub(crate)")]
    expected_files: ExpectedFiles,
}

impl FileSuccess {
    pub(super) fn new(path: RelativePathBuf, expected_files: Option<ExpectedFiles>) -> Self {
        Self {
            path,
            expected_files: expected_files.unwrap_or_default(),
        }
    }
}
