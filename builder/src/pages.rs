pub(crate) mod add;
mod index;

use ssg_child::FileSpec;

use crate::mob::{Mob, MOBS};

pub(crate) async fn all() -> impl Iterator<Item = FileSpec> {
    [index::page().await, add::page()]
        .into_iter()
        .chain(MOBS.iter().cloned().map(Mob::page))
}
