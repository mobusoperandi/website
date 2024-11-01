use maud::Render;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::From, derive_more::Display)]
pub(crate) struct RelativePathBuf(relative_path::RelativePathBuf);

impl Render for RelativePathBuf {
    fn render(&self) -> maud::Markup {
        self.0.as_str().render()
    }
}

impl From<String> for RelativePathBuf {
    fn from(path: String) -> Self {
        Self(path.into())
    }
}

impl From<&str> for RelativePathBuf {
    fn from(path: &str) -> Self {
        Self(path.into())
    }
}

impl From<RelativePathBuf> for relative_path::RelativePathBuf {
    fn from(value: RelativePathBuf) -> Self {
        value.0
    }
}
