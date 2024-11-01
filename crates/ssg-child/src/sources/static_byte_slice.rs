use futures::{future::BoxFuture, FutureExt};

use super::{FileContents, FileSource};

impl FileSource for &'static [u8] {
    fn obtain_content(
        &self,
    ) -> BoxFuture<'static, Result<FileContents, Box<dyn std::error::Error + Send>>> {
        async { Ok(FileContents::new(self.to_vec(), None)) }.boxed()
    }
}
