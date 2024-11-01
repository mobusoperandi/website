mod bytes;
mod google_font;
mod http;
mod static_byte_slice;

use std::collections::BTreeSet;

pub use bytes::*;
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
    expected_files: ExpectedFiles,
}

impl FileContents {
    #[must_use]
    pub fn new(bytes: Vec<u8>, expected_files: Option<ExpectedFiles>) -> Self {
        Self {
            bytes,
            expected_files: expected_files.unwrap_or_default(),
        }
    }

    pub(crate) fn expected_files(&self) -> Option<&ExpectedFiles> {
        if self.expected_files.is_empty() {
            None
        } else {
            Some(&self.expected_files)
        }
    }
}

#[derive(Debug, Clone, Default, derive_more::IntoIterator)]
pub struct ExpectedFiles(BTreeSet<RelativePathBuf>);

impl ExpectedFiles {
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
