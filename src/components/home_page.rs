use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use maud::{html, Markup, Render};
use ssg::sources::bytes_with_file_spec_safety::Targets;

use crate::{
    components,
    mobs::{Mob, MobId, MobTitle, Person},
    style::{BUTTON_CLASSES, BUTTON_GAP},
};

pub(crate) struct HomePage {
    pub(crate) mobs: Vec<Mob>,
    pub(crate) participants: BTreeSet<Person>,
    pub(crate) targets: Targets,
}

impl Render for HomePage {
    fn render(&self) -> maud::Markup {
        let events = self
            .mobs
            .iter()
            .flat_map(|mob| mob.events(&self.targets, event_content_template))
            .collect();

        let calendar = components::Calendar {
            targets: self.targets.clone(),
            events,
        };

        html! {
            (calendar)
            div class=(classes!("flex", "flex-wrap", format!("gap-x-{BUTTON_GAP}"))) {
                a
                    class=(*BUTTON_CLASSES)
                    href=(self.targets.path_of("/add.html").unwrap())
                    { "Add your mob" }
            }
            div class=(classes!("flex", "flex-wrap")) {
                @for person in &self.participants {
                    @if let Some(avatar_url) = &person.avatar_url {
                        a href=(person.social_url) class=(classes!("w-20")) {
                            img alt=(person.name) src=(avatar_url);
                        }
                    }
                }
            }
        }
    }
}

fn event_content_template(
    _start: DateTime<Utc>,
    _end: DateTime<Utc>,
    mob_id: MobId,
    mob_title: MobTitle,
    targets: &Targets,
) -> Markup {
    let target_path = targets.path_of(format!("/mobs/{mob_id}.html")).unwrap();
    html! {
        a class=(classes!("no-underline", "block", "h-full")) href=(target_path) {
            (mob_title)
        }
    }
}
