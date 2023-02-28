mod ident;

use anyhow::{Error, Result};

use ::schema::syn;
use maud::{html, Render};

use crate::{
    components::schema::type_::{documentation::Documentation, field::Field},
    style::FIELD_OR_VARIANT_CLASSES,
};

use self::ident::VariantIdent;

#[derive(Debug, Clone)]
pub(crate) struct Variant {
    ident: VariantIdent,
    field: Option<Field>,
    documentation: Documentation,
}

impl TryFrom<syn::Variant> for Variant {
    type Error = Error;

    fn try_from(ast_variant: syn::Variant) -> Result<Self> {
        let ident = ast_variant.ident.to_string().into();

        let ast_field = ast_variant.fields.into_iter().next();

        let field = match ast_field {
            Some(ast_field) => {
                let field: Field = ast_field.try_into()?;
                Ok::<_, Error>(Some(field))
            }
            None => Ok(None),
        }?;

        let documentation = ast_variant.attrs.try_into()?;

        Ok(Self {
            ident,
            field,
            documentation,
        })
    }
}

impl Render for Variant {
    fn render(&self) -> maud::Markup {
        html! {
            div class=(*FIELD_OR_VARIANT_CLASSES) {
                div class=(classes!("flex" "gap-[1ch]")) {
                    (self.ident)

                    @if let Some(field) = &self.field {
                        @if !field.required() { span { "optional" } }

                        (field.node())
                    }
                }
                (self.documentation)
            }
        }
    }
}
