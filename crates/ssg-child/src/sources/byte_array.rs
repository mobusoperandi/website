use futures::FutureExt;

use super::{FileContents, FileSource};

impl<const N: usize> FileSource for &[u8; N] {
    fn obtain_content(
        &self,
    ) -> futures::future::BoxFuture<Result<super::FileContents, Box<dyn std::error::Error + Send>>>
    {
        async { Ok(FileContents::new(self.to_vec(), None)) }.boxed()
    }
}
