use ::schema::syn;
use anyhow::{bail, Error, Result};
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
            .filter_map(|attr| attr.doc_string())
            .collect::<Vec<String>>();

        if doc_string_parts.is_empty() {
            bail!("no doc attrs");
        }

        let doc_string = doc_string_parts.join("\n");

        Ok(Self(Markdown::from(doc_string).to_html()))
    }
}

impl Render for Documentation {
    fn render(&self) -> maud::Markup {
        html! { div class=(*PROSE_CLASSES) { (self.0) } }
    }
}
