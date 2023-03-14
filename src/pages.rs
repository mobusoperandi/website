pub(crate) mod add;
mod index;

use std::{convert::Infallible, vec};

use futures::FutureExt;
use maud::Render;
use ssg::FileSpec;

use crate::{
    components,
    mobs::{self, Mob},
};

fn mob_page(mob: Mob) -> FileSpec {
    FileSpec::new(format!("/mobs/{}.html", mob.id), move |targets| {
        let mob = mob.clone();

        async { Ok::<_, Infallible>(components::MobPage { mob, targets }.render().0.into_bytes()) }
            .boxed()
    })
}

pub(crate) async fn all() -> Vec<FileSpec> {
    let mobs = mobs::read_all_mobs().await;
    let mut mob_pages = mobs.iter().cloned().map(mob_page).collect::<Vec<_>>();
    let mut pages = vec![index::page().await, add::page()];

    pages.append(&mut mob_pages);

    pages
}
