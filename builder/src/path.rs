use camino::{Utf8Path, Utf8PathBuf};

#[derive(Debug, Clone)]
pub(crate) struct PathBuf(Utf8PathBuf);

impl<P: AsRef<Utf8Path>> FromIterator<P> for PathBuf {
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Self(Utf8PathBuf::from_iter(iter))
    }
}

impl AsRef<std::path::Path> for PathBuf {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_ref()
    }
}

impl AsRef<str> for PathBuf {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
