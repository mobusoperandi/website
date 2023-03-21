use ::schema::syn;
use anyhow::{Error, Result};
use maud::{html, Markup, Render};

use crate::markdown::Markdown;
use crate::style::PROSE_CLASSES;
use crate::syn_helpers::Attribute;

#[derive(Debug, Clone)]
pub(crate) struct Documentation(Markup);

impl TryFrom<Vec<syn::Attribute>> for Documentation {
    type Error = Error;

    fn try_from(attrs: Vec<syn::Attribute>) -> Result<Self> {
        let doc_string_parts = attrs
            .into_iter()
            .filter_map(|attr| {
                if attr.is_doc() {
                    Some(attr.doc_string())
                } else {
                    None
                }
            })
            .collect::<Result<Vec<String>>>()?;

        let doc_string = doc_string_parts.join("\n");

        Ok(Self(Markdown::from(doc_string).to_html()))
    }
}

impl Render for Documentation {
    fn render(&self) -> maud::Markup {
        html! { div class=(*PROSE_CLASSES) { (self.0) } }
    }
}
