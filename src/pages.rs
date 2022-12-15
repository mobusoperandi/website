mod index;
mod join;
mod publish;
use super::COPYRIGHT_HOLDER;
use crate::{
    fonts,
    html::Classes,
    markdown::to_html,
    mobs::{self, Event, Mob, MobParticipant},
    NAME, REPO_URL,
};
use chrono::{Datelike, Utc};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use ssg::{Asset, Source, Targets};
use std::{path::Path, vec};

pub(crate) fn base(
    title: String,
    content: Markup,
    stylesheets: impl IntoIterator<Item = String>,
    content_classes: Classes,
    targets: &Targets,
) -> Markup {
    let version = Utc::now().timestamp_millis();
    let content_classes = content_classes + classes!["grow" "flex" "flex-col" "justify-center"];
    let markup = html! {
      (DOCTYPE)
      html lang="en" class=(classes![format!("font-[{}]", fonts::VOLLKORN) "[font-size:16px]"]) {
        head {
          title { (format!("{title}; {NAME}")) }
          meta charset="utf-8";
          meta name="viewport" content="width=device-width, initial-scale=1.0";
          link rel="stylesheet" href={ "/index.css?v=" (version) };
          @for stylesheet in stylesheets {
              link rel="stylesheet" href=(stylesheet);
          }
          style {
            // TODO extract as font utility
            @for font in fonts::ALL {(PreEscaped(format!("
              @font-face {{
                font-family: '{}';
                src: url('/{}') format('truetype');
              }}
            ", font.name, fonts::output_filename(&font))))}
          }
        }
        body class=(classes!("min-h-screen" "py-1" "px-1" "md:px-5" "flex" "flex-col" "gap-1" "max-w-screen-xl" "mx-auto")) {
            div class=(classes!("flex" "items-center" "flex-wrap" "gap-x-2" "gap-y-1" "uppercase" "text-lg")) {
                div class=(classes!("flex-1" "flex" "flex-wrap")) {
                    div class=(classes!("flex-initial" "flex" "flex-col" "gap-x-2" "whitespace-nowrap")) {
                        p class=(classes!("tracking-widest" "text-center")) { (NAME) }
                        p class=(classes!("text-sm" "text-slate-700")) { "A mob programming community" }
                    }
                }
                div class=(classes!("flex-auto" "flex" "justify-end" "flex-wrap" "gap-x-2")) {
                    a href=(targets.relative("index.html").unwrap().to_str().unwrap()) { "Calendar" }
                    a href=(targets.relative("join.html").unwrap().to_str().unwrap()) { "Join" }
                    a href=(targets.relative("publish.html").unwrap().to_str().unwrap()) { "Publish" }
                    a href="https://twitter.com/mobusoperandi" { "Twitter" }
                }
            }
            hr {}
            div class=(content_classes) {
                (content)
            }
            hr {}
            div class=(classes!("flex" "justify-between")) {
                p {
                    ({
                        let year = chrono::Utc::now().year();
                        format!("© {year} {COPYRIGHT_HOLDER}")
                    })
                    ", licensed "
                    a href="https://tldrlegal.com/license/mit-license" { "MIT" }
                    "."
                }
                a href=(REPO_URL.to_string()) { "Source"}
            }
        }
      }
    };
    markup
}

pub(crate) fn mob_page(mob: Mob) -> Asset {
    let id = mob.id.clone();
    Asset::new(
        ["mobs", &format!("{id}.html")].into_iter().collect(),
        async move {
            Source::BytesWithAssetSafety(Box::new(move |targets| {
                let (calendar_html, calendar_stylesheet) =
                    calendar(&targets, mob.events(&targets, false));
                Ok(base(
                    mob.title.clone(),
                    html! {
                        div class=(classes!("sm:grid" "grid-cols-2" "text-center" "tracking-wide")) {
                            div class=(classes!("py-12")) {
                                h1 class=(classes!("text-4xl")) { (mob.title) }
                                p {
                                    "A "
                                    a href=(targets.relative("index.html").unwrap().to_str().unwrap()) { (NAME) }
                                    " mob"
                                }
                            }
                            div class=(classes!("py-12")) {
                                h2 { "Participants" }
                                div class=(classes!("font-bold")) {
                                    @for mob_participant in mob.participants {
                                        @match mob_participant {
                                            MobParticipant::Hidden => div { "hidden participant" },
                                            MobParticipant::Public(person) => a class=(classes!("block")) href=(person.social_url.to_string()) { (person.name) },
                                        }
                                    }
                                }
                            }
                        }
                        div class=(classes!("prose")) {
                            (PreEscaped(to_html(&mob.freeform_copy_markdown)))
                        }
                        hr {}
                        (calendar_html)
                    },
                    [calendar_stylesheet],
                    classes!("gap-6"),
                    &targets,
                )
                .0
                .into_bytes())
            }))
        },
    )
}

pub(crate) async fn all() -> Vec<Asset> {
    let mobs = mobs::read_all_mobs().await;
    let mut mob_pages = mobs.iter().cloned().map(mob_page).collect::<Vec<_>>();
    let mut pages = vec![index::page().await, join::page(), publish::page()];
    pages.append(&mut mob_pages);
    pages
}

pub(crate) fn calendar(targets: &Targets, events: Vec<Event>) -> (Markup, String) {
    let events = serde_json::to_string(&events).unwrap();
    let html = html! {
        div {}
        script defer src=(targets.relative(Path::new("fullcalendar.js")).unwrap().display().to_string()) {}
        script {
            (PreEscaped(format!("window.addEventListener('DOMContentLoaded', () => {{
                const events = JSON.parse('{events}')
                {}
            }})", include_str!("pages/calendar.js"))))
        }
    };
    let stylesheet = targets
        .relative("fullcalendar.css")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    (html, stylesheet)
}
