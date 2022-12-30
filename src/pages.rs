mod index;
use crate::{
    fonts,
    html::Classes,
    markdown::to_html,
    mobs::{self, Event, Mob, MobParticipant},
    style::{self, TEXT_COLOR},
    COMMIT_HASH, DESCRIPTION, GITHUB_ORGANIZATION_URL, NAME, REPO_URL, ZULIP_URL,
};
use chrono::Utc;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use ssg::{Asset, Source, Targets};
use std::{path::Path, vec};
use url::Url;

pub(crate) fn base(
    title: String,
    content: Markup,
    stylesheets: impl IntoIterator<Item = String>,
    content_classes: Classes,
    targets: &Targets,
) -> Markup {
    let version = Utc::now().timestamp_millis();
    const NAV_ICON_SIZE: u8 = 32;
    let markup = html! {
        (DOCTYPE)
        html
        lang="en"
        class=(classes![format!("font-[{}]", fonts::VOLLKORN) "[font-size:16px]" format!("bg-{}", style::BACKGROUND_COLOR) format!("text-{}", style::TEXT_COLOR)])
        {
            head {
              title { (format!("{title}; {NAME}")) }
              meta charset="utf-8";
              meta name="description" content=(DESCRIPTION);
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
                div class=(classes!("flex" "justify-between" "items-center" "flex-wrap" "gap-x-2" "gap-y-1" "uppercase" "text-lg")) {
                    div
                        class=(classes!("flex" "flex-col" "gap-x-2" "whitespace-nowrap"))
                        {
                            a
                                href=(targets.path_of("index.html").unwrap())
                                class=(classes!("tracking-widest" "text-center"))
                                { (NAME) }
                            p class=(classes!("text-sm" "opacity-75")) { (DESCRIPTION) }
                        }
                    div class=(classes!("flex" "items-center" "gap-x-2")) {
                        a href=(ZULIP_URL.to_string()) {
                            img
                                width=(NAV_ICON_SIZE)
                                alt="Zulip"
                                src=(targets.path_of("zulip_logo.svg").unwrap());
                        }
                        a class=(classes!("invert")) href=(GITHUB_ORGANIZATION_URL.to_string()) {
                            img
                                width=(NAV_ICON_SIZE)
                                alt="GitHub"
                                src=(targets.path_of("inverticat.svg").unwrap());
                        }
                        a href="https://twitter.com/mobusoperandi" {
                            img
                                width=(NAV_ICON_SIZE)
                                alt="Twitter"
                                src=(targets.path_of("twitter_logo.svg").unwrap());
                        }
                    }
                }
                hr;
                div class=({classes!("grow") + content_classes}) {
                    (content)
                }
                hr;
                div class=(classes!("flex" "justify-between" "flex-wrap" "items-end")) {
                    pre class=(classes!("text-xs")) { code { (*COMMIT_HASH) } }
                    a class=(classes!("text-sm")) href=(REPO_URL.to_string()) { "Source"}
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
                let mob_links = mob
                    .links
                    .into_iter()
                    .map(|link| -> (Url, String) {
                        match link {
                            mobs::Link::YouTube(path) => {
                                let mut url = Url::parse("https://www.youtube.com").unwrap();
                                url.set_path(&path);
                                (url, targets.path_of("youtube_logo.svg").unwrap())
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                let mob_links = (!mob_links.is_empty()).then_some(mob_links);
                let content = html! {
                    div class=(classes!("sm:grid" "grid-cols-2" "items-center" "text-center" "tracking-wide")) {
                        div class=(classes!("py-12")) {
                            h1 class=(classes!("text-4xl")) { (mob.title) }
                            @if let Some(subtitle) = mob.subtitle {
                                p { (subtitle) }
                            }
                        }
                        div class=(classes!("py-12")) {
                            h2 { "Participants" }
                            div class=(classes!("font-bold")) {
                                @for mob_participant in mob.participants {
                                    @match mob_participant {
                                        MobParticipant::Hidden => div { "(Anonymous participant)" },
                                        MobParticipant::Public(person) => a class=(classes!("block")) href=(person.social_url.to_string()) { (person.name) },
                                    }
                                }
                            }
                        }
                    }
                    div class=(classes!("sm:grid" "grid-cols-[1fr_100px]" "gap-1" "divide-y" "sm:divide-y-0" "sm:divide-x" format!("divide-{TEXT_COLOR}"))) {
                        div class=(*style::PROSE_CLASSES) {
                            (PreEscaped(to_html(&mob.freeform_copy_markdown)))
                        }
                        @if let Some(mob_links) = mob_links {
                            div class=(classes!("p-4" "flex" "flex-col" "gap-2")) {
                                @for (url, image_path) in mob_links {
                                    a href=(url.to_string()) {
                                        img alt="YouTube" src=(image_path);
                                    }
                                }
                            }
                        }
                    }
                    hr {}
                    (calendar_html)
                };
                Ok(base(
                    mob.title.clone(),
                    content,
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
    let mut pages = vec![index::page().await];
    pages.append(&mut mob_pages);
    pages
}

pub(crate) fn calendar(targets: &Targets, events: Vec<Event>) -> (Markup, String) {
    let events = serde_json::to_string(&events).unwrap();
    let html = html! {
        div class=(classes!("[--fc-page-bg-color:transparent]")) {}
        script defer src=(targets.path_of(Path::new("fullcalendar.js")).unwrap()) {}
        script {
            (PreEscaped(format!("window.addEventListener('DOMContentLoaded', () => {{
                const events = JSON.parse('{events}')
                {}
            }})", include_str!("pages/calendar.js"))))
        }
    };
    let stylesheet = targets.path_of("fullcalendar.css").unwrap();
    (html, stylesheet)
}
