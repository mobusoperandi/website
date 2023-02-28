use maud::{html, Render};

use crate::components::schema::ident::Ident;
use crate::style::IDENT_INTENSITY;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TypeIdent(Ident);

impl TypeIdent {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<&str> for TypeIdent {
    fn eq(&self, &other: &&str) -> bool {
        self.0 == other
    }
}

impl From<String> for TypeIdent {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl Render for TypeIdent {
    fn render(&self) -> maud::Markup {
        let root_classes = classes!(format!("text-orange-{IDENT_INTENSITY}"));
        html! { span class=(root_classes) { (self.0) } }
    }
}
