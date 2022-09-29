mod in_the_media;
mod join;
mod main;
mod mightyiam;
mod mobs_calendar;
mod why_mob;
use crate::mobs::Event;
use maud::{html, Markup};
use ndarray::Array2;

pub fn sections(events: Vec<Event>) -> Array2<Section> {
    vec![
        [main::section(), join::section(), why_mob::section()],
        [
            in_the_media::section(),
            mightyiam::section(),
            mobs_calendar::section(events),
        ],
    ]
    .into()
}

pub struct Section {
    pub id: String,
    pub classes: String,
    pub stylesheet: Option<String>,
    pub content: Markup,
}

fn home(classes: String) -> Markup {
    let classes = format!("text-4xl {classes}");
    html! {
      a href="#main" class=(classes) { "🏡" }
    }
}
