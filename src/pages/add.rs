use std::ops::Deref;

use indexmap::IndexMap;
use maud::Render;
use once_cell::sync::Lazy;
use schema::{DeriveInput, Schema};
use ssg::{FileSource, FileSpec};

use crate::{
    components::{
        self,
        schema::type_::{ident::TypeIdent, Type},
    },
    mobs,
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
            mobs::MobFile::schema(),
            mobs::MobParticipant::schema(),
            mobs::Person::schema(),
            mobs::YamlRecurringSession::schema(),
            mobs::Link::schema(),
            mobs::Status::schema(),
        ]
        .map(|derive_input| (derive_input.ident.to_string().into(), derive_input.into()))
        .into_iter()
        .collect()
    });

pub fn page() -> FileSpec {
    FileSpec::new("/add.html", async {
        FileSource::BytesWithFileSpecSafety(Box::new(move |targets| {
            let internal_types = INTERNAL_TYPES_DERIVE_INPUTS
                .values()
                .map(|derive_input| Type::try_from(derive_input.deref().clone()).unwrap())
                .collect();

            let add_page = components::add_page::AddPage::new(internal_types, targets);

            Ok(add_page.render().0.into_bytes())
        }))
    })
}
