mod index;

use std::vec;

use maud::Render;
use ssg::{Asset, Source};

use crate::{
    components,
    mobs::{self, Mob},
};

fn mob_page(mob: Mob) -> Asset {
    Asset::new(
        ["/mobs", &format!("{}.html", mob.id)].into_iter().collect(),
        async move {
            Source::BytesWithAssetSafety(Box::new(move |targets| {
                Ok(components::MobPage { mob, targets }.render().0.into_bytes())
            }))
        },
    )
}

pub(crate) async fn all() -> Vec<Asset> {
    let mobs = mobs::read_all_mobs().await;
    let mut mob_pages = mobs.iter().cloned().map(mob_page).collect::<Vec<_>>();
    let mut pages = vec![index::page().await];

    pages.append(&mut mob_pages);

    pages
}
