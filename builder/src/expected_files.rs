use ssg_child::sources::ExpectedFiles;

use crate::relative_path::RelativePathBuf;

pub(crate) trait ExpectedFilesExt {
    fn insert_<P>(&mut self, path: P) -> RelativePathBuf
    where
        relative_path::RelativePathBuf: From<P>;
}

impl ExpectedFilesExt for ExpectedFiles {
    fn insert_<P>(&mut self, path: P) -> RelativePathBuf
    where
        relative_path::RelativePathBuf: From<P>,
    {
        self.insert(path).into()
    }
}
