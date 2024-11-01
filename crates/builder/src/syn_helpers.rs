pub(crate) trait Attribute {
    fn doc_string(&self) -> Option<String>;
}

impl Attribute for syn::Attribute {
    fn doc_string(&self) -> Option<String> {
        let syn::Meta::NameValue(name_value) = &self.meta else {
            return None;
        };

        let last_path_segment = name_value.path.segments.last()?;

        if last_path_segment.ident != "doc" {
            return None;
        }

        let syn::Expr::Lit(literal_expression) = &name_value.value else {
            return None;
        };

        let syn::Lit::Str(lit_str) = &literal_expression.lit else {
            return None;
        };

        Some(lit_str.value())
    }
}
