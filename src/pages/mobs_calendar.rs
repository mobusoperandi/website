use super::base;
use crate::mobs;
use maud::{html, PreEscaped};
use ssg::{Asset, Source};
use std::path::Path;

pub async fn page() -> Asset {
    let mobs = mobs::read_all_mobs().await;
    let events = mobs.into_iter().fold(Vec::new(), mobs::events);
    let events = serde_json::to_string(&events).unwrap();
    Asset::new("mobs_calendar.html".into(), async {
        Source::BytesWithAssetSafety(Box::new(move |targets| {
            let content = html! {
                div {}
                script defer src=(targets.relative(Path::new("fullcalendar.js")).unwrap().display().to_string()) {}
                script {
                    (PreEscaped(format!("window.addEventListener('DOMContentLoaded', () => {{
                        const events = JSON.parse('{events}')
                        {}
                    }})", include_str!("mobs-calendar.js"))))
                }
            };
            Ok(base(
                "Calendar".to_owned(),
                content,
                [targets
                    .relative("fullcalendar.css")
                    .unwrap()
                    .display()
                    .to_string()],
                "".into(),
                &targets,
            )
            .0
            .into_bytes())
        }))
    })
}
