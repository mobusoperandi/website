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
                    a href=(targets.relative("join.html")?.display().to_string()) {
                        "Join"
                    }
                },
                [],
                [
                    "text-center",
                    "uppercase",
                    "tracking-widest",
                    "text-4xl",
                    "leading-relaxed",
                    "sm:text-5xl",
                    "sm:leading-relaxed",
                ]
                .join(" "),
                &targets,
            )
            .0
            .into_bytes())
        }))
    })
}
