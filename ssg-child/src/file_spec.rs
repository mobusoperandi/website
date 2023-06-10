use getset::Getters;
use relative_path::RelativePathBuf;

use crate::sources::FileSource;

#[derive(Getters)]
pub struct FileSpec {
    source: Box<dyn FileSource + Send>,
    #[getset(get = "pub(crate)")]
    target: RelativePathBuf,
}

impl FileSpec {
    pub fn new<T>(target: T, source: impl FileSource + 'static + Send) -> Self
    where
        RelativePathBuf: From<T>,
    {
        Self {
            source: Box::new(source),
            target: target.into(),
        }
    }

    pub(crate) fn into_source(self) -> Box<dyn FileSource + Send> {
        self.source
    }
}
