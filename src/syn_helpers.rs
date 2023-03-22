use schema::proc_macro2;

pub(crate) trait Attribute {
    fn doc_string(&self) -> Option<String>;
}

impl Attribute for syn::Attribute {
    fn doc_string(&self) -> Option<String> {
        let last_path_segment = self.path.segments.last()?;

        if last_path_segment.ident != "doc" {
            return None;
        }

        let value_tokens = self
            .tokens
            .clone()
            .into_iter()
            .skip(1)
            .collect::<proc_macro2::TokenStream>();

        let Ok(literal) = syn::parse2::<syn::Lit>(value_tokens) else {
            return None;
        };

        let syn::Lit::Str(lit_str) = literal else {
            return None;
        };

        Some(lit_str.value())
    }
}
