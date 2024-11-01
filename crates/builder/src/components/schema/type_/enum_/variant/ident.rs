use maud::{html, Render};

use crate::{
    components::schema::ident::Ident,
    style::{IDENT_CLASSES, IDENT_INTENSITY},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VariantIdent(Ident);

impl From<String> for VariantIdent {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl Render for VariantIdent {
    fn render(&self) -> maud::Markup {
        let root_classes = IDENT_CLASSES.clone() + classes!(format!("text-red-{IDENT_INTENSITY}"));
        html! { span class=(root_classes) { "!" (self.0) } }
    }
}
