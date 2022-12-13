use super::base;
use crate::{mobs, pages::calendar};
use ssg::{Asset, Source};

pub async fn page() -> Asset {
    let mobs = mobs::read_all_mobs().await;
    Asset::new("index.html".into(), async {
        Source::BytesWithAssetSafety(Box::new(move |targets| {
            let events = mobs
                .iter()
                .flat_map(|mob| mob.events(&targets, true))
                .collect();
            let (calendar_html, calendar_stylesheet) = calendar(&targets, events);
            Ok(base(
                "Calendar".to_owned(),
                calendar_html,
                [calendar_stylesheet],
                "".into(),
                &targets,
            )
            .0
            .into_bytes())
        }))
    })
}
