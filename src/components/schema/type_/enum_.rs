mod variant;

use ::schema::syn;
use anyhow::Error;
use maud::{html, Render};

use crate::style::VERTICAL_GAP_CLASS;

use self::variant::Variant;

#[derive(Debug, Clone)]
pub(crate) struct EnumType {
    variants: Vec<Variant>,
}

impl FromIterator<Variant> for EnumType {
    fn from_iter<T: IntoIterator<Item = Variant>>(iter: T) -> Self {
        Self {
            variants: iter.into_iter().collect(),
        }
    }
}

impl TryFrom<syn::DataEnum> for EnumType {
    type Error = Error;

    fn try_from(enum_data: syn::DataEnum) -> Result<Self, Self::Error> {
        let enum_ = enum_data
            .variants
            .into_iter()
            .map(TryInto::try_into)
            .collect::<anyhow::Result<Self>>()?;

        Ok(enum_)
    }
}

impl Render for EnumType {
    fn render(&self) -> maud::Markup {
        let variant_classes = classes!["flex", "flex-col", VERTICAL_GAP_CLASS];

        html! {
            div class=(classes!("flex", "flex-col", VERTICAL_GAP_CLASS)) {
                div { "One of:" }

                ol class=(variant_classes) {
                    @for variant in &self.variants { li { (variant) } }
                }
            }
        }
    }
}
