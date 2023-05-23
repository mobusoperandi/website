use ::schema::syn;
use anyhow::{bail, Error, Result};
use maud::{html, Render};

use crate::{
    components::schema::type_::{
        documentation::Documentation,
        field::{ident::FieldIdent, Field},
    },
    style::FIELD_OR_VARIANT_CLASSES,
};

#[derive(Debug, Clone)]
pub(crate) struct MappingKey {
    ident: FieldIdent,
    field: Field,
    documentation: Documentation,
}

impl TryFrom<syn::Field> for MappingKey {
    type Error = Error;

    fn try_from(ast_field: syn::Field) -> Result<Self> {
        let Some(ident) =  &ast_field.ident else {
            bail!("expected an ident")
        };

        let ident = ident.to_string().into();

        let documentation = ast_field.attrs.clone().try_into()?;

        let field = ast_field.try_into()?;

        Ok(Self {
            ident,
            field,
            documentation,
        })
    }
}

impl Render for MappingKey {
    fn render(&self) -> maud::Markup {
        html! {
            div class=(*FIELD_OR_VARIANT_CLASSES) {
                div class=(classes!("flex", "gap-[1ch]")) {
                    (self.ident)

                    @if !self.field.required() {
                        span { "optional" }
                    }

                    (self.field.node())
                }

                (self.documentation)
            }
        }
    }
}
