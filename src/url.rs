use derive_more::FromStr;
use maud::{PreEscaped, Render};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromStr, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Url(url::Url);

impl Url {
    pub(crate) fn parse(input: &str) -> Result<Self, url::ParseError> {
        url::Url::parse(input).map(Self)
    }

    pub(crate) fn to_inner(&self) -> &url::Url {
        &self.0
    }

    pub(crate) fn set_path(&mut self, path: &str) {
        self.0.set_path(path);
    }

    pub(crate) fn path_segments_mut(&mut self) -> Result<url::PathSegmentsMut, ()> {
        self.0.path_segments_mut()
    }
}

impl Render for Url {
    fn render(&self) -> PreEscaped<String> {
        PreEscaped(self.0.to_string())
    }
}
