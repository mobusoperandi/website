use futures::{future::BoxFuture, FutureExt};

use crate::Targets;

use super::FileSource;

impl FileSource for &'static [u8] {
    fn obtain_content(
        &self,
        _targets: Targets,
    ) -> BoxFuture<'static, Result<Vec<u8>, Box<dyn std::error::Error + Send>>> {
        async { Ok(self.to_vec()) }.boxed()
    }
}
