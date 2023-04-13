pub(crate) mod ident;

use anyhow::{anyhow, bail, Error, Result};
use getset::{CopyGetters, Getters};
use schema::syn;

use crate::components::schema::node::Node;

#[derive(Debug, Clone, Getters, CopyGetters)]
pub(crate) struct Field {
    #[getset(get = "pub(crate)")]
    node: Node,
    #[getset(get_copy = "pub(crate)")]
    required: bool,
}

impl TryFrom<syn::Field> for Field {
    type Error = Error;

    fn try_from(ast_field: syn::Field) -> Result<Self> {
        let syn::Type::Path(type_path) = &ast_field.ty else {
            bail!("non-path type");
        };

        let last_path_segment = type_path
            .path
            .segments
            .last()
            .ok_or_else(|| anyhow!("empty path"))?;

        let (node, required) = if last_path_segment.ident == "Option" {
            let required = false;

            let node = last_path_segment.arguments.clone().try_into()?;

            (node, required)
        } else {
            let required = true;

            let node = last_path_segment.clone().try_into()?;

            (node, required)
        };

        Ok(Self { node, required })
    }
}
