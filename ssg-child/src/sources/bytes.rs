use futures::{future::BoxFuture, FutureExt};

use super::{ExpectedTargets, FileContents, FileSource};

#[derive(Debug)]
pub struct BytesSource {
    bytes: Vec<u8>,
    expected_targets: ExpectedTargets,
}

impl BytesSource {
    pub fn new(bytes: Vec<u8>, expected_targets: Option<ExpectedTargets>) -> Self {
        let expected_targets = expected_targets.unwrap_or_default();

        Self {
            bytes,
            expected_targets,
        }
    }
}

impl FileSource for BytesSource {
    fn obtain_content(
        &self,
    ) -> BoxFuture<'static, Result<FileContents, Box<dyn std::error::Error + Send>>> {
        let bytes = self.bytes.clone();
        let expected_targets = Some(self.expected_targets.clone());
        async { Ok(FileContents::new(bytes, expected_targets)) }.boxed()
    }
}
