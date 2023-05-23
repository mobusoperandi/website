use maud::{Markup, PreEscaped, Render};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, derive_more::Display, Serialize, Deserialize)]
pub(crate) struct Title(String);

impl Title {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Render for Title {
    fn render(&self) -> Markup {
        PreEscaped(self.0.clone())
    }
}
