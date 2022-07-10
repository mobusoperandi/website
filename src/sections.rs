mod in_the_media;
mod join;
mod main;
mod mightyiam;
mod mobs_calendar;
mod why_mob;
use maud::{html, Markup};
use ndarray::Array2;

pub fn sections() -> Array2<Section> {
    vec![
        [main::section(), join::section(), why_mob::section()],
        [
            in_the_media::section(),
            mightyiam::section(),
            mobs_calendar::section(),
        ],
    ]
    .into()
}

pub struct Section {
    id: String,
    classes: String,
    stylesheet: Option<String>,
    content: Markup,
}

impl Section {
    pub fn new(id: String, classes: String, stylesheet: Option<String>, content: Markup) -> Self {
        Self {
            id,
            classes,
            stylesheet,
            content,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn classes(&self) -> &str {
        &self.classes
    }
    pub fn stylesheet(&self) -> Option<&str> {
        self.stylesheet.as_deref()
    }
    pub fn content(&self) -> Markup {
        self.content.clone()
    }
}

fn home(classes: String) -> Markup {
    let classes = format!("text-4xl {classes}");
    html! {
      a href="#main" class=(classes) { "🏡" }
    }
}
