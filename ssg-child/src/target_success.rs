use relative_path::RelativePathBuf;

#[derive(Debug, Clone)]
pub struct TargetSuccess(RelativePathBuf);

impl TargetSuccess {
    pub(super) fn new(path: RelativePathBuf) -> Self {
        Self(path)
    }
}
