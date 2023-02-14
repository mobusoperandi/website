use chrono::Utc;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use ssg::Targets;

use crate::{
    constants::{COMMIT_HASH, DESCRIPTION, GITHUB_ORGANIZATION_URL, NAME, REPO_URL, ZULIP_URL},
    fonts,
    html::Classes,
    mobs::MobTitle,
    style,
};

pub(crate) struct BasePage {
    pub(crate) title: Option<MobTitle>,
    pub(crate) content: Markup,
    pub(crate) content_classes: Classes,
    pub(crate) targets: Targets,
}

impl Render for BasePage {
    fn render(&self) -> maud::Markup {
        let version = Utc::now().timestamp_millis();
        let title = self
            .title
            .clone()
            .map_or(NAME.to_owned(), |title| format!("{title}; {NAME}"));
        const NAV_ICON_SIZE: u8 = 32;
        let brand_classes = classes!("tracking-widest" "text-center");
        let target_index = self.targets.path_of("/index.html").unwrap();
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
                                @if target_index == self.targets.current_path() {
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
                                    src=(self.targets.path_of("/zulip_logo.svg").unwrap());
                            }
                            a class=(classes!("invert")) href=(*GITHUB_ORGANIZATION_URL) {
                                img
                                    width=(NAV_ICON_SIZE)
                                    alt="GitHub"
                                    src=(self.targets.path_of("/inverticat.svg").unwrap());
                            }
                            a href="https://twitter.com/mobusoperandi" {
                                img
                                    width=(NAV_ICON_SIZE)
                                    alt="Twitter"
                                    src=(self.targets.path_of("/twitter_logo.svg").unwrap());
                            }
                        }
                    }
                    hr;
                    div class=({self.content_classes.clone() + classes!("grow")}) {
                        (self.content)
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
}
