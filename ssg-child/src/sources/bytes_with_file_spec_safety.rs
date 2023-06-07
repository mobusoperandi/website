use std::collections::BTreeSet;

use futures::{future::BoxFuture, FutureExt, TryFutureExt};
use relative_path::{RelativePath, RelativePathBuf};

use super::FileSource;

impl<T, E> FileSource for T
where
    T: Fn(Targets) -> BoxFuture<'static, Result<Vec<u8>, E>> + Send,
    E: std::error::Error + 'static + Send,
{
    fn obtain_content(
        &self,
        targets: Targets,
    ) -> BoxFuture<'static, Result<Vec<u8>, Box<dyn std::error::Error + Send>>> {
        self(targets)
            .map_err(|error| -> Box<dyn std::error::Error + Send> { Box::new(error) })
            .boxed()
    }
}

#[derive(Debug, Clone)]
pub struct Targets {
    current: RelativePathBuf,
    all: BTreeSet<RelativePathBuf>,
}

#[derive(Debug, thiserror::Error)]
#[error("target not found: {target}")]
pub struct TargetNotFoundError {
    target: RelativePathBuf,
}

impl TargetNotFoundError {
    pub fn new(target: RelativePathBuf) -> Self {
        Self { target }
    }
}

impl Targets {
    pub(crate) fn new(current: RelativePathBuf, all: BTreeSet<RelativePathBuf>) -> Self {
        Self { current, all }
    }

    pub fn path_of_(
        &self,
        path: impl AsRef<RelativePath>,
    ) -> Result<RelativePathBuf, TargetNotFoundError> {
        let path = path.as_ref();

        if !self.all.contains(path) {
            return Err(TargetNotFoundError::new(path.to_owned()));
        }

        Ok(if path == "index.html" {
            RelativePathBuf::from("/")
        } else {
            path.to_owned()
        })
    }

    pub fn current_path_(&self) -> RelativePathBuf {
        self.path_of_(&self.current).unwrap()
    }
}
