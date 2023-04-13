use chrono::DateTime;
use maud::{html, Markup, Render};
use ssg::sources::bytes_with_file_spec_safety::{TargetNotFoundError, Targets};

use crate::components::CalendarEvent;
use crate::mob::Participant;
use crate::{
    components,
    constants::NAME,
    mob::{self, Mob},
    style,
    url::Url,
};

use super::PageBase;

pub(crate) struct MobPage {
    pub(crate) mob: Mob,
    pub(crate) links: Vec<(Url, String, &'static str)>,
    pub(crate) events: Vec<CalendarEvent>,
    pub(crate) base: PageBase,
    pub(crate) fullcalendar_path: String,
    pub(crate) rrule_path: String,
    pub(crate) fullcalendar_rrule_path: String,
}

impl Render for MobPage {
    fn render(&self) -> maud::Markup {
        type WrapperFn = fn(&str) -> Markup;

        fn status_wrapper_false(content: &str) -> Markup {
            html!(s class=(classes!("opacity-70")) { (content) })
        }

        fn status_wrapper_true(content: &str) -> Markup {
            html!(span { (content) })
        }

        let join_content = match &self.mob.status {
            mob::Status::Short(join_content) => Some(join_content.to_owned()),
            mob::Status::Open(join_content) => Some(join_content.to_owned()),
            mob::Status::Full(join_content) => join_content.to_owned(),
            mob::Status::Public(join_content) => Some(join_content.to_owned()),
        };

        let calendar = components::Calendar {
            events: self.events.clone(),
            status_legend: None,
            fullcalendar_path: self.fullcalendar_path.clone(),
            rrule_path: self.rrule_path.clone(),
            fullcalendar_rrule_path: self.fullcalendar_rrule_path.clone(),
        };

        let (short_wrapper, open_wrapper, full_wrapper, public_wrapper): (
            WrapperFn,
            WrapperFn,
            WrapperFn,
            WrapperFn,
        ) = match self.mob.status {
            mob::Status::Short(_) => (
                status_wrapper_true,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
            ),
            mob::Status::Open(_) => (
                status_wrapper_false,
                status_wrapper_true,
                status_wrapper_false,
                status_wrapper_false,
            ),
            mob::Status::Full(_) => (
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_true,
                status_wrapper_false,
            ),
            mob::Status::Public(_) => (
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_true,
            ),
        };

        let root_classes = classes!(
            "flex",
            "flex-col",
            "sm:flex-row",
            "sm:justify-around",
            "text-center",
            "tracking-wide"
        );

        let content = html! {
            div class=(root_classes) {
                div class=(classes!("py-12")) {
                    h1 class=(classes!("text-4xl")) { (self.mob.title) }
                    @if let Some(subtitle) = &self.mob.subtitle {
                        (subtitle)
                    }
                }

                @if !self.links.is_empty() {
                    div class=(classes!("flex", "sm:flex-col", "justify-center", "gap-2")) {
                        @for (url, image_path, alt) in &self.links {
                            a href=(url) {
                                img class=(classes!("h-8")) alt=(alt) src=(image_path);
                            }
                        }
                    }
                }

                div class=(classes!("py-12")) {
                    h2 { "Participants" }
                    div class=(classes!("font-bold")) {
                        @for mob_participant in &self.mob.participants {
                            @match mob_participant {
                                Participant::Hidden => div { "(Anonymous participant)" },
                                Participant::Public(person) => a class=(classes!("block")) href=(person.social_url) { (person.name) },
                            }
                        }
                    }
                }
            }

            div class=(classes!("flex", "flex-col", "items-center", "gap-1", "text-lg")) {
                div class=(classes!("flex", "gap-4", "uppercase", "tracking-widest")) {
                    (short_wrapper("short")) (open_wrapper("open")) (full_wrapper("full")) (public_wrapper("public"))
                }
                p class="tracking-wide" { (mob::Status::description(self.mob.status.as_ref())) }
            }

            div class=(classes!("grid", "grid-flow-row", "sm:grid-flow-col", "auto-cols-fr", "gap-[1.25em]")) {
                div class=(*style::PROSE_CLASSES) {
                    (self.mob.freeform_copy_markdown)
                }
                div class=(*style::PROSE_CLASSES) {
                    @if let Some(join_content) = join_content {
                        (join_content)
                    }
                }
            }

            hr;

            (calendar)
        };

        self.base
            .clone()
            .into_page(
                Some(self.mob.title.as_str().to_owned().into()),
                content,
                classes!("flex", "flex-col", "gap-6"),
                components::page_base::PageDescription::from(format!(
                    "{}{}; description, schedule and more on {NAME}",
                    self.mob.title,
                    self.mob
                        .subtitle
                        .as_ref()
                        .map(|subtitle| format!(", {}", subtitle.as_str()))
                        .unwrap_or_default()
                )),
            )
            .render()
    }
}

pub(crate) fn event_content_template(
    start: DateTime<rrule::Tz>,
    end: DateTime<rrule::Tz>,
    _mob: &Mob,
    _targets: &Targets,
) -> Result<Markup, TargetNotFoundError> {
    let start = start.format("%k:%M").to_string();
    let end = end.format("%k:%M").to_string();
    let content = html! {
        (start) "–" (end) " UTC"
    };
    Ok(content)
}
