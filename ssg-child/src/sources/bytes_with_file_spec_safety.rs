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

    pub fn path_of(&self, path: impl AsRef<Utf8Path>) -> Result<String, TargetNotFoundError> {
        let path = path.as_ref();

        assert!(path.is_absolute(), "path not absolute: {path:?}");

        self.all
            .contains(path)
            .then(|| Utf8PathBuf::from_iter([Utf8PathBuf::from("/"), path.to_owned()]).to_string())
            .map(|path| {
                if path == "/index.html" {
                    String::from("/")
                } else {
                    path
                }
            })
            .ok_or_else(|| TargetNotFoundError::new(path.to_owned()))
    }

    pub fn current_path(&self) -> String {
        self.path_of(&self.current).unwrap()
    }
}
