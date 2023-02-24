use comrak::markdown_to_html;
use maud::{Markup, PreEscaped};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Markdown(String);

impl Markdown {
    pub(crate) fn to_html(&self) -> Markup {
        PreEscaped(markdown_to_html(&self.0, &Default::default()))
    }
}

impl From<String> for Markdown {
    fn from(s: String) -> Self {
        Self(s)
    }
}
