use anyhow::{bail, Result};
use schema::proc_macro2;

pub(crate) trait Attribute {
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
