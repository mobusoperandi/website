use chrono::{DateTime, Utc};
use maud::{html, Markup, Render};
use ssg::sources::bytes_with_file_spec_safety::Targets;

use crate::{
    components,
    constants::NAME,
    mobs::{self, Mob, MobParticipant},
    style,
    url::Url,
};

pub(crate) struct MobPage {
    pub(crate) mob: Mob,
    pub(crate) targets: Targets,
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
            mobs::Status::Short(join_content) => Some(join_content.to_owned()),
            mobs::Status::Open(join_content) => Some(join_content.to_owned()),
            mobs::Status::Full(join_content) => join_content.to_owned(),
            mobs::Status::Public(join_content) => Some(join_content.to_owned()),
        };

        let events = self.mob.events(&self.targets, event_content_template);
        let calendar = components::Calendar {
            targets: self.targets.clone(),
            events,
            status_legend: None,
        };

        let mob_links = self
            .mob
            .links
            .clone()
            .into_iter()
            .map(|link| match link {
                mobs::Link::YouTube(path) => {
                    let mut url = Url::parse("https://www.youtube.com").unwrap();
                    url.set_path(&path);
                    (
                        url,
                        self.targets.path_of("/youtube_logo.svg").unwrap(),
                        "YouTube",
                    )
                }
            })
            .collect::<Vec<_>>();

        let mob_links = (!mob_links.is_empty()).then_some(mob_links);

        let (short_wrapper, open_wrapper, full_wrapper, public_wrapper): (
            WrapperFn,
            WrapperFn,
            WrapperFn,
            WrapperFn,
        ) = match self.mob.status {
            mobs::Status::Short(_) => (
                status_wrapper_true,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
            ),
            mobs::Status::Open(_) => (
                status_wrapper_false,
                status_wrapper_true,
                status_wrapper_false,
                status_wrapper_false,
            ),
            mobs::Status::Full(_) => (
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_true,
                status_wrapper_false,
            ),
            mobs::Status::Public(_) => (
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

                @if let Some(mob_links) = mob_links {
                    div class=(classes!("flex", "sm:flex-col", "justify-center", "gap-2")) {
                        @for (url, image_path, alt) in mob_links {
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
                                MobParticipant::Hidden => div { "(Anonymous participant)" },
                                MobParticipant::Public(person) => a class=(classes!("block")) href=(person.social_url) { (person.name) },
                            }
                        }
                    }
                }
            }

            div class=(classes!("flex", "flex-col", "items-center", "gap-1", "text-lg")) {
                div class=(classes!("flex", "gap-4", "uppercase", "tracking-widest")) {
                    (short_wrapper("short")) (open_wrapper("open")) (full_wrapper("full")) (public_wrapper("public"))
                }
                p class="tracking-wide" { (mobs::Status::description(self.mob.status.as_ref())) }
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

        components::BasePage {
            title: Some(self.mob.title.as_str().to_owned().into()),
            content,
            description: components::base_page::PageDescription::from(format!(
                "{}{}; description, schedule and more on {NAME}",
                self.mob.title,
                self.mob
                    .subtitle
                    .as_ref()
                    .map(|subtitle| format!(", {}", subtitle.as_str()))
                    .unwrap_or_default()
            )),
            content_classes: classes!("flex", "flex-col", "gap-6"),
            targets: self.targets.clone(),
        }
        .render()
    }
}

fn event_content_template(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    _mob: &Mob,
    _targets: &Targets,
) -> Markup {
    let start = start.format("%k:%M").to_string();
    let end = end.format("%k:%M").to_string();
    html! {
        (start) "–" (end) " UTC"
    }
}
