use std::collections::HashMap;

use anyhow::{anyhow, bail, Error, Result};
use maud::{html, Render};
use once_cell::sync::Lazy;
use schema::syn;

use crate::{pages::add::INTERNAL_TYPES_DERIVE_INPUTS, url::Url};

use super::type_::{ident::TypeIdent, link::TypeLink};

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Seq(Box<Node>),
    InternalType(TypeIdent),
    ExternalType { ident: TypeIdent, url: Url },
    Scalar,
}

static EXTERNAL_TYPES: Lazy<HashMap<TypeIdent, Url>> = Lazy::new(|| {
    [
        (
            "Color",
            "https://developer.mozilla.org/en-US/docs/Web/CSS/color_value",
        ),
        (
            "Url",
            "https://developer.mozilla.org/en-US/docs/Glossary/URL",
        ),
        (
            "Tz",
            "https://en.wikipedia.org/wiki/List_of_tz_database_time_zones",
        ),
        (
            "Markdown",
            "https://docs.github.com/en/get-started\
            /writing-on-github\
            /getting-started-with-writing-and-formatting-on-github\
            /basic-writing-and-formatting-syntax",
        ),
    ]
    .map(|(ident, url)| (TypeIdent::from(ident.to_owned()), Url::parse(url).unwrap()))
    .into_iter()
    .collect()
});

impl TryFrom<syn::PathSegment> for Node {
    type Error = Error;

    fn try_from(path_segment: syn::PathSegment) -> Result<Self> {
        let ident: TypeIdent = path_segment.ident.to_string().into();

        let node = if ident == "Vec" {
            Self::Seq(Box::new(path_segment.arguments.try_into()?))
        } else if INTERNAL_TYPES_DERIVE_INPUTS.contains_key(&ident) {
            Self::InternalType(ident)
        } else if let Some(url) = EXTERNAL_TYPES.get(&ident).cloned() {
            Self::ExternalType { ident, url }
        } else {
            Self::Scalar
        };

        Ok(node)
    }
}

impl TryFrom<syn::PathArguments> for Node {
    type Error = Error;

    fn try_from(path_arguments: syn::PathArguments) -> std::result::Result<Self, Self::Error> {
        let syn::PathArguments::AngleBracketed(path_arguments) = path_arguments else {
            bail!("Option with non angle bracketed arguments")
        };

        let some_type = path_arguments
            .args
            .first()
            .ok_or_else(|| anyhow!("doesn't have an argument"))?;

        let syn::GenericArgument::Type(some_type) = some_type else {
            bail!("type argument is not a type")
        };

        let syn::Type::Path(type_path) = some_type else {
            bail!("non-path type");
        };

        type_path
            .path
            .segments
            .last()
            .cloned()
            .ok_or_else(|| anyhow!("empty path"))?
            .try_into()
    }
}

impl Render for Node {
    fn render(&self) -> maud::Markup {
        match self {
            Self::Seq(node) => html! { span {"sequence of " (node)} },
            Self::InternalType(ident) => TypeLink::from(ident.clone()).render(),
            Self::ExternalType { ident, url } => html! { a href=(url) { (ident) } },
            Self::Scalar => TypeIdent::from("scalar".to_owned()).render(),
        }
    }
}
