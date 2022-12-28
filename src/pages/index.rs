use super::base;
use crate::{mobs, pages::calendar, style::BUTTON_CLASSES, DEFAULT_BRANCH, MOBS_PATH, REPO_URL};
use maud::html;
use ssg::{Asset, Source};

pub async fn page() -> Asset {
    let mobs = mobs::read_all_mobs().await;
    Asset::new("index.html".into(), async {
        let mut existing_mobs_url = REPO_URL.clone();
        existing_mobs_url
            .path_segments_mut()
            .unwrap()
            .push("tree")
            .push(&DEFAULT_BRANCH)
            .push(MOBS_PATH);
        Source::BytesWithAssetSafety(Box::new(move |targets| {
            let events = mobs
                .iter()
                .flat_map(|mob| mob.events(&targets, true))
                .collect();
            let (calendar_html, calendar_stylesheet) = calendar(&targets, events);
            Ok(base(
                "Calendar".to_owned(),
                html! {
                    (calendar_html)
                    div class=(classes!("flex" "flex-wrap" "gap-2")) {
                        a
                            class=(*BUTTON_CLASSES)
                            href=(existing_mobs_url.to_string())
                            { "Add your mob" }
                    }
                },
                [calendar_stylesheet],
                classes!("flex" "flex-col" "gap-1"),
                &targets,
            )
            .0
            .into_bytes())
        }))
    })
}
