use anyhow::{bail, Error, Result};
use maud::{html, Render};
use schema::syn;

use crate::style::{GRAYS, VERTICAL_GAP_CLASS};

use self::{documentation::Documentation, enum_::EnumType, ident::TypeIdent, mapping::MappingType};

mod documentation;
mod enum_;
mod field;
pub(crate) mod ident;
pub(crate) mod link;
mod mapping;

#[derive(Debug, Clone)]
pub(crate) struct Type {
    ident: TypeIdent,
    data: TypeData,
    documentation: Documentation,
}

#[derive(Debug, Clone)]
pub(crate) enum TypeData {
    Map(MappingType),
    Enum(EnumType),
}

impl TryFrom<syn::DeriveInput> for Type {
    type Error = Error;

    fn try_from(derive_input: syn::DeriveInput) -> Result<Self, Self::Error> {
        let ident = derive_input.ident.to_string().into();

        let data = match derive_input.data {
            syn::Data::Struct(struct_data) => TypeData::Map(struct_data.try_into()?),
            syn::Data::Enum(enum_data) => TypeData::Enum(enum_data.try_into()?),
            syn::Data::Union(_) => bail!("only struct or enum"),
        };

        let documentation = derive_input.attrs.try_into()?;

        Ok(Self {
            ident,
            data,
            documentation,
        })
    }
}

impl Render for Type {
    fn render(&self) -> maud::Markup {
        let data_html = match &self.data {
            TypeData::Map(map_type) => map_type.render(),
            TypeData::Enum(enum_type) => enum_type.render(),
        };

        let root_classes = classes!(
            format!("bg-{GRAYS}-800"),
            "p-1",
            "flex",
            "flex-col",
            VERTICAL_GAP_CLASS
        );

        html! {
            div id=(self.ident.as_str()) class=(root_classes) {
                h2 class=(classes!("text-xl")) { (self.ident) }

                (self.documentation)

                (data_html)
            }
        }
    }
}
