use ssg_child::sources::ExpectedTargets;

use crate::relative_path::RelativePathBuf;

pub(crate) trait ExpectedTargetsExt {
    fn insert_<P>(&mut self, path: P) -> RelativePathBuf
    where
        relative_path::RelativePathBuf: From<P>;
}

impl ExpectedTargetsExt for ExpectedTargets {
    fn insert_<P>(&mut self, path: P) -> RelativePathBuf
    where
        relative_path::RelativePathBuf: From<P>,
    {
        self.insert(path).into()
    }
}
