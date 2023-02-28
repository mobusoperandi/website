use ::schema::syn;
use anyhow::{bail, Error, Result};
use maud::{html, Markup, Render};
use schema::proc_macro2;

use crate::markdown::Markdown;
use crate::style::PROSE_CLASSES;

#[derive(Debug, Clone)]
pub(crate) struct Documentation(Markup);

trait Attribute {
    fn is_doc(&self) -> bool;
    fn doc_string(&self) -> Result<String>;
}

impl Attribute for syn::Attribute {
    fn is_doc(&self) -> bool {
        if !matches!(self.style, syn::AttrStyle::Outer) {
            return false;
        }

        let last_path_segment = self.path.segments.last();

        let Some(last_path_segment) = last_path_segment else {
            return false;
        };

        last_path_segment.ident == "doc"
    }

    fn doc_string(&self) -> Result<String> {
        if !self.is_doc() {
            bail!("not a doc attr");
        }

        let value_tokens = self
            .tokens
            .clone()
            .into_iter()
            .skip(1)
            .collect::<proc_macro2::TokenStream>();

        let Ok(literal) = syn::parse2::<syn::Lit>(value_tokens) else {
            bail!("not a literal");
        };

        let syn::Lit::Str(lit_str) = literal else {
            bail!("not a Lit::Str");
        };

        Ok(lit_str.value())
    }
}

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
