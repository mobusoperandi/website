use super::base;
use crate::NAME;
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
                            a href="https://mobusoperandi.zulipchat.com/"
                            { "Join our discussion platform" }
                            "."
                        }
                        li {
                            a
                                href=(targets.relative("calendar.html")?
                                      .display()
                                      .to_string())
                                { "See existing mobs" }
                            "."
                        }
                        li {
                            a
                                href=(targets.relative("publish.html")?
                                      .display()
                                      .to_string())
                                { "Publish a new mob" }
                            "."
                        }
                    }
                },
                [],
                ["prose", "tracking-wide", "text-xl"].join(" "),
                &targets,
            )
            .0
            .into_bytes())
        }))
    })
}
