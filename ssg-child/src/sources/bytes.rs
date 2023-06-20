use futures::{future::BoxFuture, FutureExt};

use super::{ExpectedFiles, FileContents, FileSource};

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct BytesSource {
    bytes: Vec<u8>,
    expected_files: ExpectedFiles,
}

impl BytesSource {
    #[must_use]
    pub fn new(bytes: Vec<u8>, expected_files: Option<ExpectedFiles>) -> Self {
        let expected_files = expected_files.unwrap_or_default();

        Self {
            bytes,
            expected_files,
        }
    }
}

impl FileSource for BytesSource {
    fn obtain_content(
        &self,
    ) -> BoxFuture<'static, Result<FileContents, Box<dyn std::error::Error + Send>>> {
        let bytes = self.bytes.clone();
        let expected_files = Some(self.expected_files.clone());
        async { Ok(FileContents::new(bytes, expected_files)) }.boxed()
    }
}
