pub(super) mod legend;

use custom_attrs::CustomAttrs;
use maud::{Markup, Render};
use schema::Schema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumVariantNames, VariantNames};

use crate::{markdown::Markdown, syn_helpers::Attribute};

pub(crate) use self::legend::Legend;

#[derive(Debug, Clone, Serialize, Deserialize, Schema, AsRefStr, EnumVariantNames, CustomAttrs)]
#[attr(indicator: Option<char>)]
/// A mob's status
pub(crate) enum Status {
    /// This mob is not active yet because it needs more members.
    ///
    /// The value explains how to apply.
    ///
    /// Example:
    ///
    /// ```yaml
    /// !Short |
    ///   To apply contact [Kelly](https://example.com/kelly).
    /// ```
    #[attr(indicator = '🌱')]
    Short(Markdown),
    /// This mob is taking applications for new participants.
    ///
    /// The value explains how to apply.
    ///
    /// Example:
    ///
    /// ```yaml
    /// !Open |
    ///   To apply contact [Dawn](https://example.com/dawn).
    /// ```
    #[attr(indicator = '👐')]
    Open(Markdown),
    /// This mob is not currently taking applications.
    ///
    /// The value is optional.
    ///     
    /// Example:
    ///
    /// ```yaml
    /// !Full |
    ///   We are currently full.
    /// ```
    Full(Option<Markdown>),
    /// This mob's sessions are open for anyone to join.
    ///
    /// The value explains how to join.
    ///
    /// Example:
    ///
    /// ```yaml
    /// !Public |
    ///   [Room link](https://meet.jit.si/MedievalWebsPortrayLoud)
    /// ```
    #[attr(indicator = '⛲')]
    Public(Markdown),
}

impl Status {
    pub(crate) fn description(variant_ident: &str) -> Description {
        let syn::Data::Enum(enum_data) = Self::schema().data else {
            panic!("not an enum??")
        };

        let variant = enum_data
            .variants
            .into_iter()
            .find(|variant| variant.ident == variant_ident)
            .expect("variant not found");

        let description = variant
            .attrs
            .into_iter()
            .find_map(|attr| attr.doc_string())
            .expect("no doc attr");

        Description(description)
    }

    pub(crate) fn indicator(&self) -> Option<StatusIndicator> {
        Some(StatusIndicator(self.get_indicator()?))
    }

    pub(crate) fn indicator_for_ident(variant_ident: &str) -> Option<StatusIndicator> {
        let syn::Data::Enum(enum_data) = Self::schema().data else {
            panic!("not an enum??")
        };

        let variant = enum_data
            .variants
            .into_iter()
            .find(|variant| variant.ident == variant_ident)
            .expect("variant not found");

        let list_tokens = variant.attrs.into_iter().find_map(|attr| {
            let syn::Meta::List(list_meta) = attr.meta else {
                return None;
            };

            if !list_meta.path.is_ident("attr") {
                return None;
            };

            Some(list_meta.tokens)
        })?;

        let name_value = syn::parse2::<syn::MetaNameValue>(list_tokens).ok()?;

        if !name_value.path.is_ident("indicator") {
            return None;
        }

        let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Char(lit_char),
            ..
        }) = name_value.value
        else {
            panic!("value is not a literal");
        };

        Some(StatusIndicator(lit_char.value()))
    }

    pub(crate) fn legend() -> Legend {
        Self::VARIANTS
            .iter()
            .filter_map(|&variant_ident| {
                let indicator = Self::indicator_for_ident(variant_ident)?;
                let description = Self::description(variant_ident);

                Some((indicator, description))
            })
            .collect::<Legend>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct StatusIndicator(char);

impl Render for StatusIndicator {
    fn render(&self) -> Markup {
        self.0.render()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Description(String);

impl Render for Description {
    fn render(&self) -> Markup {
        self.0.render()
    }
}
