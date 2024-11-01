use maud::{html, Render};

use crate::components::schema::ident::Ident;
use crate::style::IDENT_INTENSITY;

#[derive(Debug, Clone)]
pub(crate) struct FieldIdent(Ident);

impl From<String> for FieldIdent {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl Render for FieldIdent {
    fn render(&self) -> maud::Markup {
        let root_classes = classes!("font-bold", format!("text-blue-{IDENT_INTENSITY}"));

        html! {
            span class=(root_classes) { (self.0) }
        }
    }
}
