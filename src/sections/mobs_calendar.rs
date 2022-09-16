use super::{home, Section};
use crate::mobs::Event;
use maud::{html, PreEscaped};

pub fn section(events: Vec<Event>) -> Section {
    let events = serde_json::to_string(&events).unwrap();
    Section::new(
        "mobs_calendar".into(),
        "".into(),
        Some("https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.css".into()),
        html! {
            (home("row-start-1 column-start-5 column-end-7".into()))
            div class="row-start-2 row-end-7 col-span-full" {}
            script defer src="https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.js" {}
            script defer {
                (PreEscaped(format!("const events = JSON.parse('{events}');")))
                (PreEscaped(include_str!("mobs-calendar.js")))
            }
        },
    )
}
