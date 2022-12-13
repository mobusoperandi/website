mod index;
mod join;
mod publish;
use super::COPYRIGHT_HOLDER;
use crate::{
    fonts,
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
    content_classes: String,
    targets: &Targets,
) -> Markup {
    let version = Utc::now().timestamp_millis();
    let content_classes =
        content_classes + " " + &["grow", "flex", "flex-col", "justify-center"].join(" ");
    let markup = html! {
      (DOCTYPE)
      html lang="en" class=(format!("font-[{}] [font-size:16px]", fonts::VOLLKORN)) {
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
        body."min-h-screen"."p-1".flex."flex-col"."gap-1" {
            div.flex."items-center"."flex-wrap"."gap-x-2"."gap-y-1".uppercase."text-lg" {
                div."flex-1".flex."flex-wrap" {
                    div."flex-initial"."flex"."flex-col"."gap-x-2"."whitespace-nowrap" {
                        p."tracking-widest"."text-center" { (NAME) }
                        p."text-sm"."text-slate-700" { "A mob programming community" }
                    }
                }
                div."flex-auto".flex."justify-end"."flex-wrap"."gap-x-2" {
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
            div.flex."justify-between" {
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
                        div."sm:grid"."grid-cols-2"."text-center"."tracking-wide" {
                            div."py-12" {
                                h1."text-4xl" { (mob.title) }
                                p {
                                    "A "
                                    a href=(targets.relative("index.html").unwrap().to_str().unwrap()) { (NAME) }
                                    " mob"
                                }
                            }
                            div."py-12" {
                                h2 { "Participants" }
                                div."font-bold" {
                                    @for mob_participant in mob.participants {
                                        @match mob_participant {
                                            MobParticipant::Hidden => div { "hidden participant" },
                                            MobParticipant::Public(person) => a.block href=(person.social_url.to_string()) { (person.name) },
                                        }
                                    }
                                }
                            }
                        }
                        div.prose {
                            (PreEscaped(to_html(&mob.freeform_copy_markdown)))
                        }
                        hr {}
                        (calendar_html)
                    },
                    [calendar_stylesheet],
                    "gap-6".to_string(),
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
