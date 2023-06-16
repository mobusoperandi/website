pub mod bytes;
pub mod google_font;
pub mod http;
mod static_byte_slice;

use std::collections::BTreeSet;

use futures::future::BoxFuture;
use getset::Getters;
pub use google_font::*;
pub use http::*;
use relative_path::RelativePathBuf;

pub trait FileSource {
    fn obtain_content(&self) -> BoxFuture<Result<FileContents, Box<dyn std::error::Error + Send>>>;
}

#[derive(Debug, Getters)]
pub struct FileContents {
    #[getset(get = "pub(crate)")]
    bytes: Vec<u8>,
    expected_targets: ExpectedTargets,
}

impl FileContents {
    pub fn new(bytes: Vec<u8>, expected_targets: Option<ExpectedTargets>) -> Self {
        Self {
            bytes,
            expected_targets: expected_targets.unwrap_or_default(),
        }
    }

    pub(crate) fn expected_targets(&self) -> Option<&ExpectedTargets> {
        if self.expected_targets.is_empty() {
            None
        } else {
            Some(&self.expected_targets)
        }
    }
}

#[derive(Debug, Clone, Default, derive_more::IntoIterator)]
pub struct ExpectedTargets(BTreeSet<RelativePathBuf>);

impl ExpectedTargets {
    pub fn insert<P>(&mut self, path: P) -> RelativePathBuf
    where
        RelativePathBuf: From<P>,
    {
        let path = RelativePathBuf::from(path);
        self.0.insert(path.clone());
        path
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
