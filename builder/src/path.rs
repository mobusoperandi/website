use camino::{Utf8Path, Utf8PathBuf};
use maud::Render;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::From, derive_more::Display)]
pub(crate) struct PathBuf(Utf8PathBuf);

impl From<String> for PathBuf {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

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

impl Render for PathBuf {
    fn render(&self) -> maud::Markup {
        self.0.as_str().render()
    }
}
