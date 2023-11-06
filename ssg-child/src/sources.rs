mod bytes;
mod google_font;
mod http;
mod static_byte_slice;

use std::{collections::BTreeSet, convert::Infallible};

pub use bytes::*;
use futures::future::BoxFuture;
use getset::Getters;
pub use google_font::*;
pub use http::*;
use relative_path::RelativePathBuf;
use serde::{Serialize, Deserialize};

pub trait FileSource {
    fn obtain_content(&self) -> BoxFuture<Result<FileContents, Box<dyn std::error::Error + Send>>>;
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FileSourceEnum<C = Infallible> {
    Bytes(BytesSource),
    GoogleFont(GoogleFont),
    Http(Http),
    StaticByteSlice(&'static [u8]),
    Custom(C),
}

impl FileSource for Infallible {
    fn obtain_content(&self) -> BoxFuture<Result<FileContents, Box<dyn std::error::Error + Send>>> {
        match *self {}
    }
}

impl<C: FileSource> FileSource for FileSourceEnum<C> {
    fn obtain_content(&self) -> BoxFuture<Result<FileContents, Box<dyn std::error::Error + Send>>> {
        match self {
            FileSourceEnum::Bytes(source) => source.obtain_content(),
            FileSourceEnum::GoogleFont(source) => source.obtain_content(),
            FileSourceEnum::Http(source) => source.obtain_content(),
            FileSourceEnum::StaticByteSlice(source) => source.obtain_content(),
            FileSourceEnum::Custom(source) => source.obtain_content(),
        }
    }
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

#[derive(Debug, Clone, Default, derive_more::IntoIterator, Serialize, Deserialize)]
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
