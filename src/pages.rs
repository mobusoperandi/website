mod index;
use crate::{
    calendar,
    constants::{COMMIT_HASH, DESCRIPTION, GITHUB_ORGANIZATION_URL, NAME, REPO_URL, ZULIP_URL},
    fonts,
    html::Classes,
    markdown::to_html,
    mobs::{self, Mob, MobParticipant},
    style,
    url::Url,
};
use chrono::Utc;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use ssg::{Asset, Source, Targets};
use std::vec;

pub(crate) fn base(
    title: Option<String>,
    content: Markup,
    content_classes: Classes,
    targets: &Targets,
) -> Markup {
    let version = Utc::now().timestamp_millis();
    let title = title.map_or(NAME.to_owned(), |title| format!("{title}; {NAME}"));
    const NAV_ICON_SIZE: u8 = 32;
    let brand_classes = classes!("tracking-widest" "text-center");
    let target_index = targets.path_of("/index.html").unwrap();
    let markup = html! {
        (DOCTYPE)
        html
        lang="en"
        class=(classes![format!("font-[{}]", fonts::VOLLKORN) "[font-size:16px]" format!("bg-{}", style::BACKGROUND_COLOR) format!("text-{}", style::TEXT_COLOR)])
        {
            head {
              title { (title) }
              meta charset="utf-8";
              meta name="description" content=(DESCRIPTION);
              meta name="viewport" content="width=device-width, initial-scale=1.0";
              link rel="stylesheet" href={ "/index.css?v=" (version) };
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
                            @if target_index == targets.current_path() {
                                p
                                    class=(brand_classes)
                                    { (NAME) }
                            } @else {
                                a
                                    href=(target_index)
                                    class=(brand_classes)
                                    { (NAME) }
                            }
                            p class=(classes!("text-sm" "opacity-75")) { (DESCRIPTION) }
                        }
                    div class=(classes!("flex" "items-center" "gap-x-2")) {
                        a href=(*ZULIP_URL) {
                            img
                                width=(NAV_ICON_SIZE)
                                alt="Zulip"
                                src=(targets.path_of("/zulip_logo.svg").unwrap());
                        }
                        a class=(classes!("invert")) href=(*GITHUB_ORGANIZATION_URL) {
                            img
                                width=(NAV_ICON_SIZE)
                                alt="GitHub"
                                src=(targets.path_of("/inverticat.svg").unwrap());
                        }
                        a href="https://twitter.com/mobusoperandi" {
                            img
                                width=(NAV_ICON_SIZE)
                                alt="Twitter"
                                src=(targets.path_of("/twitter_logo.svg").unwrap());
                        }
                    }
                }
                hr;
                div class=({content_classes + classes!("grow")}) {
                    (content)
                }
                hr;
                div class=(classes!("flex" "justify-between" "flex-wrap" "items-end")) {
                    pre class=(classes!("text-xs")) { code { (*COMMIT_HASH) } }
                    a class=(classes!("text-sm")) href=(*REPO_URL) { "Source"}
                }
            }
        }
    };
    markup
}

pub(crate) fn mob_page(mob: Mob) -> Asset {
    type WrapperFn = fn(&str) -> Markup;
    fn status_wrapper_false(content: &str) -> Markup {
        html!(s class=(classes!("opacity-70")) { (content) })
    }
    fn status_wrapper_true(content: &str) -> Markup {
        html!(span { (content) })
    }
    let id = mob.id.clone();
    Asset::new(
        ["/mobs", &format!("{id}.html")].into_iter().collect(),
        async move {
            Source::BytesWithAssetSafety(Box::new(move |targets| {
                let join_content = match &mob.status {
                    mobs::Status::Short(join_content) => Some(join_content.clone()),
                    mobs::Status::Open(join_content) => Some(join_content.clone()),
                    mobs::Status::Full(join_content) => join_content.clone(),
                    mobs::Status::Public(join_content) => Some(join_content.clone()),
                };
                let events = mob.events(false, false);
                let calendar_html = calendar::markup(&targets, events, true);
                let mob_links = mob
                    .links
                    .into_iter()
                    .map(|link| match link {
                        mobs::Link::YouTube(path) => {
                            let mut url = Url::parse("https://www.youtube.com").unwrap();
                            url.set_path(&path);
                            (
                                url,
                                targets.path_of("/youtube_logo.svg").unwrap(),
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
                ) = match mob.status {
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
                            h1 class=(classes!("text-4xl")) { (mob.title) }
                            @if let Some(subtitle) = mob.subtitle {
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
                                @for mob_participant in mob.participants {
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
                            (PreEscaped(to_html(&mob.freeform_copy_markdown)))
                        }
                        div class=(*style::PROSE_CLASSES) {
                            @if let Some(join_content) = join_content {
                                (PreEscaped(to_html(&join_content)))
                            }
                        }
                    }
                    hr;
                    (calendar_html)
                };
                Ok(base(
                    Some(mob.title.clone()),
                    content,
                    classes!("flex" "flex-col" "gap-6"),
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
