use std::path::Path;

use super::{home, Section};
use crate::mobs::Event;
use maud::{html, PreEscaped};
use ssg::Targets;

pub fn section(targets: Targets, events: Vec<Event>) -> Section {
    let events = serde_json::to_string(&events).unwrap();
    Section {
        id: "mobs_calendar".into(),
        classes: "".into(),
        stylesheet: Some(
            targets
                .relative(Path::new("fullcalendar.css"))
                .unwrap()
                .display()
                .to_string(),
        ),
        content: html! {
            (home("row-start-1 column-start-5 column-end-7".into()))
            div class="row-start-2 row-end-7 col-span-full" {}
            script defer src=(targets.relative(Path::new("fullcalendar.js")).unwrap().display().to_string()) {}
            script {
                (PreEscaped(format!("window.addEventListener('DOMContentLoaded', () => {{
                    const events = JSON.parse('{events}')
                    {}
                }})", include_str!("mobs-calendar.js"))))
            }
        },
    }
}
