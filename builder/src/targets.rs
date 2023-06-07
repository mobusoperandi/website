use relative_path::RelativePath;
use ssg_child::sources::bytes_with_file_spec_safety::{TargetNotFoundError, Targets};

use crate::relative_path::RelativePathBuf;

pub(crate) trait TargetsExt {
    fn path_of(
        &self,
        path: impl AsRef<RelativePath>,
    ) -> Result<RelativePathBuf, TargetNotFoundError>;
    fn current_path(&self) -> RelativePathBuf;
}

impl TargetsExt for Targets {
    fn path_of(
        &self,
        path: impl AsRef<RelativePath>,
    ) -> Result<RelativePathBuf, TargetNotFoundError> {
        self.path_of_(path.as_ref()).map(|path| path.into())
    }

    fn current_path(&self) -> RelativePathBuf {
        self.current_path_().into()
    }
}
