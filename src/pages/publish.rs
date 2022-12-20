use super::base;
use crate::{style, DEFAULT_BRANCH, MOBS_PATH, REPO_URL};
use maud::html;
use ssg::{Asset, Source};

pub fn page() -> Asset {
    Asset::new("publish.html".into(), async {
        Source::BytesWithAssetSafety(Box::new(|targets| {
            let mut existing_mobs_url = REPO_URL.clone();
            existing_mobs_url
                .path_segments_mut()
                .unwrap()
                .push("tree")
                .push(&DEFAULT_BRANCH)
                .push(MOBS_PATH);
            Ok(base(
                "Publish".to_owned(),
                html! {
                    h1 { "How to publish a mob" }
                    p { "To publish a mob submit a pull request." }
                    p {
                        "See "
                        a href=(existing_mobs_url.to_string()) { "existing mob data" }
                        " for examples."
                    }
                },
                [],
                style::PROSE_CLASSES.clone() + classes!("tracking-wide" "text-xl"),
                &targets,
            )
            .0
            .into_bytes())
        }))
    })
}
