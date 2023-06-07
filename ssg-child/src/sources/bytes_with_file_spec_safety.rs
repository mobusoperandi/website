use std::collections::BTreeSet;

use camino::{Utf8Path, Utf8PathBuf};
use futures::{future::BoxFuture, FutureExt, TryFutureExt};

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
    current: Utf8PathBuf,
    all: BTreeSet<Utf8PathBuf>,
}

#[derive(Debug, thiserror::Error)]
#[error("target not found: {target}")]
pub struct TargetNotFoundError {
    target: Utf8PathBuf,
}

impl TargetNotFoundError {
    pub fn new(target: Utf8PathBuf) -> Self {
        Self { target }
    }
}

impl Targets {
    pub(crate) fn new(current: Utf8PathBuf, all: BTreeSet<Utf8PathBuf>) -> Self {
        Self { current, all }
    }

    pub fn path_of_(&self, path: impl AsRef<Utf8Path>) -> Result<Utf8PathBuf, TargetNotFoundError> {
        let path = path.as_ref();

        assert!(path.is_absolute(), "path not absolute: {path:?}");

        if !self.all.contains(path) {
            return Err(TargetNotFoundError::new(path.to_owned()));
        }

        Ok(if path == "/index.html" {
            Utf8PathBuf::from("/")
        } else {
            path.to_owned()
        })
    }

    pub fn current_path_(&self) -> Utf8PathBuf {
        self.path_of_(&self.current).unwrap()
    }
}
