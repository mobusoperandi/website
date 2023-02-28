use maud::Render;
use ssg::{Asset, Source};

use crate::{components, mobs};

pub async fn page() -> Asset {
    let mobs = mobs::read_all_mobs().await;
    let participants = mobs::get_all_participants().await;

    Asset::new("/index.html", async {
        Source::BytesWithAssetSafety(Box::new(move |targets| {
            let base_page = components::BasePage {
                title: None,
                content: components::HomePage {
                    targets: targets.clone(),
                    mobs,
                    participants,
                }
                .render(),
                content_classes: classes!("flex", "flex-col", "gap-1"),
                targets,
            };

            Ok(base_page.render().0.into_bytes())
        }))
    })
}
