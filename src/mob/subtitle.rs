use maud::{html, Markup, Render};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, derive_more::Display, Serialize, Deserialize)]
pub(crate) struct Subtitle(String);

impl Subtitle {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Render for Subtitle {
    fn render(&self) -> Markup {
        html! { p { (self.0) } }
    }
}
