use chrono::{DateTime, Utc};
use maud::{html, Markup, PreEscaped, Render};
use ssg::Targets;

use crate::{
    components,
    markdown::to_html,
    mobs::{self, Mob, MobId, MobParticipant, MobTitle},
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
            mobs::Status::Short(join_content) => Some(join_content.clone()),
            mobs::Status::Open(join_content) => Some(join_content.clone()),
            mobs::Status::Full(join_content) => join_content.clone(),
            mobs::Status::Public(join_content) => Some(join_content.clone()),
        };
        let events = self.mob.events(&self.targets, event_content_template);
        let calendar = components::Calendar {
            targets: self.targets.clone(),
            events,
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
        let (short_wrapper, open_wrapper, full_wrapper, public_wrapper, status_explanation): (
            WrapperFn,
            WrapperFn,
            WrapperFn,
            WrapperFn,
            Markup,
        ) = match self.mob.status {
            mobs::Status::Short(_) => (
                status_wrapper_true,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
                html!("This mob needs more participants to become active."),
            ),
            mobs::Status::Open(_) => (
                status_wrapper_false,
                status_wrapper_true,
                status_wrapper_false,
                status_wrapper_false,
                html!("This mob is open to more participants."),
            ),
            mobs::Status::Full(_) => (
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_true,
                status_wrapper_false,
                html!("This mob is not interested in more participants at this time."),
            ),
            mobs::Status::Public(_) => (
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_false,
                status_wrapper_true,
                html!("This mob is public, so anyone can join."),
            ),
        };
        let content = html! {
            div class=(classes!("flex" "flex-col" "sm:flex-row" "sm:justify-around" "text-center" "tracking-wide")) {
                div class=(classes!("py-12")) {
                    h1 class=(classes!("text-4xl")) { (self.mob.title) }
                    @if let Some(subtitle) = &self.mob.subtitle {
                        p { (subtitle) }
                    }
                }
                @if let Some(mob_links) = mob_links {
                    div class=(classes!("flex" "sm:flex-col" "justify-center" "gap-2")) {
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
            div class=(classes!("flex" "flex-col" "items-center" "gap-1" "text-lg")) {
                div class=(classes!("flex" "gap-4" "uppercase" "tracking-widest")) {
                    (short_wrapper("short")) (open_wrapper("open")) (full_wrapper("full")) (public_wrapper("public"))
                }
                p class="tracking-wide" { (status_explanation) }
            }
            div class=(classes!("grid" "grid-flow-row" "sm:grid-flow-col" "auto-cols-fr" "gap-[1.25em]")) {
                div class=(*style::PROSE_CLASSES) {
                    (PreEscaped(to_html(&self.mob.freeform_copy_markdown)))
                }
                div class=(*style::PROSE_CLASSES) {
                    @if let Some(join_content) = join_content {
                        (PreEscaped(to_html(&join_content)))
                    }
                }
            }
            hr;
            (calendar)
        };
        components::BasePage {
            title: Some(self.mob.title.clone()),
            content,
            content_classes: classes!("flex" "flex-col" "gap-6"),
            targets: self.targets.clone(),
        }
        .render()
    }
}

fn event_content_template(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    _mob_id: MobId,
    _mob_title: MobTitle,
    _targets: &Targets,
) -> Markup {
    let start = start.format("%k:%M").to_string();
    let end = end.format("%k:%M").to_string();
    html! {
        (start) "–" (end) " UTC"
    }
}
