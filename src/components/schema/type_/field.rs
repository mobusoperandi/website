pub(crate) mod ident;

use anyhow::{anyhow, bail, Error, Result};
use schema::syn;

use crate::components::schema::node::Node;

#[derive(Debug, Clone)]
pub(crate) struct Field {
    node: Node,
    required: bool,
}

impl Field {
    pub(crate) fn node(&self) -> &Node {
        &self.node
    }

    pub(crate) fn required(&self) -> bool {
        self.required
    }
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
