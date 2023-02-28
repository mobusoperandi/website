pub(super) mod key;

use ::schema::syn;
use anyhow::{Error, Result};
use maud::{html, Render};

use crate::style::VERTICAL_GAP_CLASS;

use self::key::MappingKey;

#[derive(Debug, Clone)]
pub(crate) struct MappingType {
    keys: Vec<MappingKey>,
}

impl TryFrom<syn::DataStruct> for MappingType {
    type Error = Error;

    fn try_from(struct_data: syn::DataStruct) -> Result<Self, Self::Error> {
        let keys = struct_data
            .fields
            .into_iter()
            .map(MappingKey::try_from)
            .collect::<Result<Vec<MappingKey>>>()?;

        Ok(Self { keys })
    }
}

impl Render for MappingType {
    fn render(&self) -> maud::Markup {
        html! {
            ol class=(classes!("flex" "flex-col" VERTICAL_GAP_CLASS)) {
                @for key in &self.keys { li { (key) } }
            }
        }
    }
}
