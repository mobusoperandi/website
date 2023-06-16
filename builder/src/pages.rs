pub(crate) mod add;
mod index;

use anyhow::Result;
use ssg_child::FileSpec;

use crate::mob::{Mob, MOBS};

pub(crate) fn all() -> Result<impl Iterator<Item = FileSpec>> {
    Ok([index::page(), add::page()?]
        .into_iter()
        .chain(MOBS.iter().cloned().map(Mob::page)))
}
