use std::ops::Deref;

use futures::FutureExt;
use indexmap::IndexMap;
use maud::Render;
use once_cell::sync::Lazy;
use schema::{DeriveInput, Schema};
use ssg_child::{
    sources::bytes_with_file_spec_safety::{TargetNotFoundError, Targets},
    FileSpec,
};

use crate::{
    components::{
        self,
        schema::type_::{ident::TypeIdent, Type},
    },
    mob,
};

#[derive(Clone)]
pub(crate) struct DeriveInputWrapper(DeriveInput);

unsafe impl Sync for DeriveInputWrapper {}
unsafe impl Send for DeriveInputWrapper {}

impl From<DeriveInput> for DeriveInputWrapper {
    fn from(derive_input: DeriveInput) -> Self {
        Self(derive_input)
    }
}

impl std::ops::Deref for DeriveInputWrapper {
    type Target = DeriveInput;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) static INTERNAL_TYPES_DERIVE_INPUTS: Lazy<IndexMap<TypeIdent, DeriveInputWrapper>> =
    Lazy::new(|| {
        [
            mob::MobFile::schema(),
            mob::Participant::schema(),
            mob::Person::schema(),
            mob::YamlRecurringSession::schema(),
            mob::Link::schema(),
            mob::Status::schema(),
        ]
        .map(|derive_input| (derive_input.ident.to_string().into(), derive_input.into()))
        .into_iter()
        .collect()
    });

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
enum AddPageError {
    Anyhow(#[from] anyhow::Error),
    TargetNotFound(#[from] TargetNotFoundError),
}

pub fn page() -> FileSpec {
    FileSpec::new("/add.html", move |targets: Targets| {
        async move {
            let internal_types = INTERNAL_TYPES_DERIVE_INPUTS
                .values()
                .map(|derive_input| Type::try_from(derive_input.deref().clone()))
                .collect::<Result<Vec<Type>, anyhow::Error>>()?;

            let base = components::PageBase::new(targets.clone())?;
            let add_page = components::add_page::AddPage::new(internal_types, base);

            Ok::<_, AddPageError>(add_page.render().0.into_bytes())
        }
        .boxed()
    })
}
