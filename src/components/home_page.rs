use std::collections::BTreeSet;

use chrono::DateTime;
use itertools::Itertools;
use maud::{html, Markup, Render};

use ssg::sources::bytes_with_file_spec_safety::{TargetNotFoundError, Targets};

use crate::components::CalendarEvent;
use crate::constants::DESCRIPTION;
use crate::{
    components,
    html::Class,
    mob::{self, Mob, Person},
    style::{BUTTON_CLASSES, BUTTON_GAP},
};

use super::PageBase;

pub(crate) struct HomePage {
    pub(crate) participants: BTreeSet<Person>,
    pub(crate) status_legend: mob::StatusLegend,
    pub(crate) events: Vec<CalendarEvent>,
    pub(crate) base: PageBase,
    pub(crate) add_page_path: String,
    pub(crate) fullcalendar_path: String,
    pub(crate) rrule_path: String,
    pub(crate) fullcalendar_rrule_path: String,
}

impl Render for HomePage {
    fn render(&self) -> maud::Markup {
        let calendar = components::Calendar {
            events: self.events.clone(),
            status_legend: Some(self.status_legend.clone()),
            fullcalendar_path: self.fullcalendar_path.clone(),
            rrule_path: self.rrule_path.clone(),
            fullcalendar_rrule_path: self.fullcalendar_rrule_path.clone(),
        };

        let content = html! {
            (calendar)
            div class=(classes!("flex", "flex-wrap", format!("gap-x-{BUTTON_GAP}"))) {
                a
                class=(*BUTTON_CLASSES)
                    href=(self.add_page_path)
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
        };

        let page = self.base.clone().into_page(
            None,
            content,
            classes!("flex", "flex-col", "gap-1"),
            components::page_base::PageDescription::from(DESCRIPTION.to_owned()),
        );

        page.render()
    }
}

pub(crate) fn event_content_template(
    _start: DateTime<rrule::Tz>,
    _end: DateTime<rrule::Tz>,
    mob: &Mob,
    targets: &Targets,
) -> Result<Markup, TargetNotFoundError> {
    let mob_id = &mob.id;
    let target_path = targets.path_of(format!("/mobs/{mob_id}.html"))?;

    const OFFSET_VALUES: [i8; 2] = [-1, 1];

    let indicator_text_shadow_value = OFFSET_VALUES
        .into_iter()
        .cartesian_product(OFFSET_VALUES)
        .map(|(offset_x, offset_y)| format!("{offset_x}px {offset_y}px 3px black"))
        .join(", ");

    let indicator_text_shadow_class: Class =
        format!("[text-shadow: {indicator_text_shadow_value}]")
            .replace(' ', "_")
            .try_into()
            .unwrap();

    let content = html! {
        a class=(classes!("no-underline", "block", "h-full")) href=(target_path) {
            (mob.title)

            @if let Some(status_indicator) = mob.status.indicator() {
                 " " span class=(indicator_text_shadow_class) { (status_indicator) }
            }
        }
    };

    Ok(content)
}
