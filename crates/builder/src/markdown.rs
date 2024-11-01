use comrak::{markdown_to_html, ComrakOptions};
use maud::{Markup, PreEscaped, Render};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Markdown(String);

impl Markdown {
    fn to_html(&self) -> Markup {
        PreEscaped(markdown_to_html(&self.0, &ComrakOptions::default()))
    }
}

impl From<String> for Markdown {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Render for Markdown {
    fn render(&self) -> Markup {
        self.to_html()
    }
}
