pub(crate) mod add;
mod index;

use std::vec;

use ssg::FileSpec;

use crate::mobs::{Mob, MOBS};

pub(crate) async fn all() -> Vec<FileSpec> {
    let mut mob_pages = MOBS.iter().cloned().map(Mob::page).collect::<Vec<_>>();
    let mut pages = vec![index::page().await, add::page()];

    pages.append(&mut mob_pages);

    pages
}
