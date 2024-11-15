pub(crate) mod add;
mod index;

use ssg_child::FileSpec;

use crate::mob::Mob;

pub(crate) fn all(mobs: Vec<Mob>) -> impl Iterator<Item = FileSpec> {
    [index::page(&mobs), add::page()]
        .into_iter()
        .chain(mobs.into_iter().map(|m| Mob::page(m)))
}
