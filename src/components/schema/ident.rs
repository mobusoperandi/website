use maud::{html, Render};

use crate::style::IDENT_CLASSES;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Ident(String);

impl Ident {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl From<String> for Ident {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl Render for Ident {
    fn render(&self) -> maud::Markup {
        html! { span class=(*IDENT_CLASSES) { (self.0) } }
    }
}
