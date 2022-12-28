use super::base;
use crate::{style, NAME, ZULIP_URL};
use maud::html;
use ssg::{Asset, Source};

pub fn page() -> Asset {
    Asset::new("join.html".into(), async {
        Source::BytesWithAssetSafety(Box::new(|targets| {
            Ok(base(
                "Join".to_owned(),
                html! {
                    h1 { (format!("Join {NAME}")) }
                    ul {
                        li {
                            a href=(ZULIP_URL.to_string())
                            { "Join our chat platform" }
                            "."
                        }
                        li {
                            a
                                href=(targets.path_of("index.html")?)
                                { "See existing mobs" }
                            "."
                        }
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
