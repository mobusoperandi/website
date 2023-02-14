use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use maud::{html, Markup, Render};
use ssg::Targets;

use crate::{
    components,
    constants::{DEFAULT_BRANCH, MOBS_PATH, REPO_URL},
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
        let mut existing_mobs_url = REPO_URL.clone();
        existing_mobs_url
            .path_segments_mut()
            .unwrap()
            .push("tree")
            .push(DEFAULT_BRANCH)
            .push(MOBS_PATH);

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
            div class=(classes!("flex" "flex-wrap" format!("gap-x-{BUTTON_GAP}"))) {
                a
                    class=(*BUTTON_CLASSES)
                    href=(existing_mobs_url)
                    { "Add your mob" }
            }
            div class=(classes!("flex" "flex-wrap")) {
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
        a class=(classes!("no-underline" "block" "h-full")) href=(target_path) {
            (mob_title)
        }
    }
}
