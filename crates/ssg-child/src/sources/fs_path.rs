use std::path::Path;

use super::{FileContents, FileSource};
use futures::{future::BoxFuture, FutureExt, TryFutureExt};

impl FileSource for Path {
    fn obtain_content(
        &self,
    ) -> BoxFuture<'static, Result<FileContents, Box<dyn std::error::Error + Send>>> {
        let path = self.to_owned();
        async move {
            let bytes = tokio::fs::read(path).await?;
            Ok(FileContents::new(bytes, None))
        }
        .map_err(|error: std::io::Error| -> Box<dyn std::error::Error + Send> { Box::new(error) })
        .boxed()
    }
}
