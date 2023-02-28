use maud::{html, Render};

use super::ident::TypeIdent;

#[derive(Debug, Clone)]
pub(crate) struct TypeLink(TypeIdent);

impl From<TypeIdent> for TypeLink {
    fn from(type_ident: TypeIdent) -> Self {
        Self(type_ident)
    }
}

impl Render for TypeLink {
    fn render(&self) -> maud::Markup {
        html! { a href=(format!("#{}", self.0.as_str())) { (self.0) } }
    }
}
