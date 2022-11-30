use super::base;
use maud::html;
use ssg::{Asset, Source};

pub fn page() -> Asset {
    Asset::new("index.html".into(), async {
        Source::BytesWithAssetSafety(Box::new(|targets| {
            Ok(base(
                "🏠".to_owned(),
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
                        "See calendar"
                    }
                },
                [],
                [
                    "grow",
                    "text-center",
                    "uppercase",
                    "tracking-widest",
                    "text-4xl",
                    "leading-relaxed",
                    "flex",
                    "flex-col",
                    "justify-around",
                ]
                .join(" "),
                &targets,
            )
            .0
            .into_bytes())
        }))
    })
}
