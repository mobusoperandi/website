use chrono::DateTime;
use maud::{html, Markup, Render};

use ssg_child::sources::ExpectedTargets;

use crate::components::CalendarEvent;
use crate::mob::{LinkElement, Participant};
use crate::relative_path::RelativePathBuf;
use crate::{
    components,
    constants::NAME,
    mob::{self, Mob},
    style,
};

use super::PageBase;

pub(crate) struct MobPage {
    mob: Mob,
    links: Vec<LinkElement>,
    events: Vec<CalendarEvent>,
    base: PageBase,
    fullcalendar_path: RelativePathBuf,
    rrule_path: RelativePathBuf,
    fullcalendar_rrule_path: RelativePathBuf,
}

impl MobPage {
    pub(crate) fn new(
        mob: Mob,
        links: Vec<LinkElement>,
        events: Vec<CalendarEvent>,
        base: PageBase,
        fullcalendar_path: RelativePathBuf,
        rrule_path: RelativePathBuf,
        fullcalendar_rrule_path: RelativePathBuf,
    ) -> Self {
        Self {
            mob,
            links,
            events,
            base,
            fullcalendar_path,
            rrule_path,
            fullcalendar_rrule_path,
        }
    }
}

impl Render for MobPage {
    #[allow(clippy::too_many_lines)]
    fn render(&self) -> maud::Markup {
        type WrapperFn = fn(&str) -> Markup;

        fn status_wrapper_false(content: &str) -> Markup {
            html!(s class=(classes!("opacity-70")) { (content) })
        }

        fn status_wrapper_true(content: &str) -> Markup {
            html!(span { (content) })
        }

        let join_content = match self.mob.status() {
            mob::Status::Short(join_content) | mob::Status::Open(join_content) => {
                Some(join_content.clone())
            }
            mob::Status::Full(join_content) => join_content.clone(),
            mob::Status::Public(join_content) => Some(join_content.clone()),
        };

        let calendar = components::Calendar::new(
            self.events.clone(),
            None,
            self.fullcalendar_path.clone(),
            self.rrule_path.clone(),
            self.fullcalendar_rrule_path.clone(),
        );

        let (short_wrapper, open_wrapper, full_wrapper, public_wrapper): (
            WrapperFn,
            WrapperFn,
            WrapperFn,
            WrapperFn,
        ) = match self.mob.status() {
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
                    h1 class=(classes!("text-4xl")) { (self.mob.title()) }
                    @if let Some(subtitle) = &self.mob.subtitle() {
                        (subtitle)
                    }
                }

                @if !self.links.is_empty() {
                    div class=(classes!("flex", "sm:flex-col", "justify-center", "gap-2")) {
                        @for link in &self.links {
                            (link)
                        }
                    }
                }

                div class=(classes!("py-12")) {
                    h2 { "Participants" }
                    div class=(classes!("font-bold")) {
                        @for mob_participant in self.mob.participants() {
                            @match mob_participant {
                                Participant::Hidden => div { "(Anonymous participant)" },
                                Participant::Public(person) => a class=(classes!("block")) href=(person.social_url()) { (person.name()) },
                            }
                        }
                    }
                }
            }

            div class=(classes!("flex", "flex-col", "items-center", "gap-1", "text-lg")) {
                div class=(classes!("flex", "gap-4", "uppercase", "tracking-widest")) {
                    (short_wrapper("short")) (open_wrapper("open")) (full_wrapper("full")) (public_wrapper("public"))
                }
                p class="tracking-wide" { (mob::Status::description(self.mob.status().as_ref())) }
            }

            div class=(classes!("grid", "grid-flow-row", "sm:grid-flow-col", "auto-cols-fr", "gap-[1.25em]")) {
                div class=(*style::PROSE_CLASSES) {
                    (self.mob.freeform_copy_markdown())
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
                Some(self.mob.title().as_str().to_owned().into()),
                content,
                classes!("flex", "flex-col", "gap-6"),
                components::page_base::PageDescription::from(format!(
                    "{}{}; description, schedule and more on {NAME}",
                    self.mob.title(),
                    self.mob
                        .subtitle()
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
    _expected_targets: &mut ExpectedTargets,
) -> Markup {
    let start = start.format("%k:%M").to_string();
    let end = end.format("%k:%M").to_string();
    let content = html! {
        (start) "–" (end) " UTC"
    };
    content
}
