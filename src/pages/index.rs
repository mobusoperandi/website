use super::base;
use maud::html;
use ssg::{Asset, Source};

pub fn page() -> Asset {
    Asset::new("index.html".into(), async {
        Source::BytesWithAssetSafety(Box::new(|targets| {
            Ok(base(
                html! {
                    h2 {
                        span { "Study" }
                        " "
                        span { "software development" }
                        " "
                        span { "online" }
                        " in "
                        span { "mob programming" }
                        " format."
                    }
                    a href=(targets.relative("mobs_calendar.html")?.display().to_string()) {
                        "See existing mobs"
                    }
                },
                [],
                [
                    "grow",
                    "text-center",
                    "uppercase",
                    "tracking-widest",
                    "text-5xl",
                    "leading-relaxed",
                    "flex",
                    "flex-col",
                    "justify-around",
                ]
                .join(" "),
            )
            .0
            .into_bytes())
        }))
    })
}
