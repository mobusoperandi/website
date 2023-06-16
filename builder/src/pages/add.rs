use std::ops::Deref;

use anyhow::Result;
use indexmap::IndexMap;
use maud::Render;
use once_cell::sync::Lazy;
use schema::{DeriveInput, Schema};
use ssg_child::{
    sources::{bytes::BytesSource, ExpectedTargets},
    FileSpec,
};

use crate::{
    components::{
        self,
        schema::type_::{ident::TypeIdent, Type},
    },
    mob,
    relative_path::RelativePathBuf,
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

pub fn page() -> Result<FileSpec> {
    let current_path = RelativePathBuf::from("/add.html");

    let internal_types = INTERNAL_TYPES_DERIVE_INPUTS
        .values()
        .map(|derive_input| Type::try_from(derive_input.deref().clone()))
        .collect::<Result<Vec<Type>, anyhow::Error>>()?;

    let mut expected_targets = ExpectedTargets::default();
    let base = components::PageBase::new(&mut expected_targets, current_path.clone());
    let add_page = components::add_page::AddPage::new(internal_types, base);

    let bytes = add_page.render().0.into_bytes();

    Ok(FileSpec::new(
        current_path,
        BytesSource::new(bytes, Some(expected_targets)),
    ))
}
