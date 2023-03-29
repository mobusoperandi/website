use std::fmt::Display;

use chrono::Utc;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use ssg::sources::bytes_with_file_spec_safety::Targets;

use crate::{
    constants::{COMMIT_HASH, DESCRIPTION, GITHUB_ORGANIZATION_URL, NAME, REPO_URL, ZULIP_URL},
    fonts,
    html::Classes,
    style,
};

#[derive(Debug, Clone)]
pub(crate) struct PageTitle(String);

impl Render for PageTitle {
    fn render(&self) -> Markup {
        self.0.render()
    }
}

impl Display for PageTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for PageTitle {
    fn from(string: String) -> Self {
        Self(string)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PageDescription(String);

impl Render for PageDescription {
    fn render(&self) -> Markup {
        self.0.render()
    }
}

impl From<String> for PageDescription {
    fn from(string: String) -> Self {
        Self(string)
    }
}

pub(crate) struct PageBase {
    title: Option<PageTitle>,
    content: Markup,
    content_classes: Classes,
    targets: Targets,
    description: PageDescription,
}

impl PageBase {
    pub(crate) fn new(
        title: Option<PageTitle>,
        content: Markup,
        content_classes: Classes,
        targets: Targets,
        description: PageDescription,
    ) -> Self {
        Self {
            title,
            content,
            content_classes,
            targets,
            description,
        }
    }
}

impl Render for PageBase {
    fn render(&self) -> Markup {
        let version = Utc::now().timestamp_millis();

        let title = self
            .title
            .clone()
            .map_or(NAME.to_owned(), |title| format!("{title}; {NAME}"));

        const NAV_ICON_SIZE: u8 = 32;

        let brand_classes = classes!("tracking-widest", "text-center");
        let target_index = self.targets.path_of("/index.html").unwrap();

        let root_classes = classes![
            format!("font-[{}]", *fonts::VOLLKORN),
            "[font-size:16px]",
            format!("bg-{}", style::BACKGROUND_COLOR),
            format!("text-{}", style::TEXT_COLOR),
        ];

        let body_classes = classes!(
            "min-h-screen",
            "py-1",
            "px-1",
            "md:px-5",
            "flex",
            "flex-col",
            "gap-1",
            "max-w-screen-xl",
            "mx-auto"
        );
        let header_classes = classes!(
            "flex",
            "justify-between",
            "items-center",
            "flex-wrap",
            "gap-x-2",
            "gap-y-1",
            "uppercase",
            "text-lg"
        );

        let markup = html! {
            (DOCTYPE)
            html
            lang="en"
            class=(root_classes)
            {
                head {
                  title { (title) }
                  meta charset="utf-8";
                  meta name="description" content=(self.description);
                  meta name="viewport" content="width=device-width, initial-scale=1.0";
                  link rel="stylesheet" href={ "/index.css?v=" (version) };
                  style {
                    // TODO extract as font utility
                    @for font in fonts::ALL.as_slice() {
                        (PreEscaped(format!("
                            @font-face {{
                                font-family: '{}';
                                src: url('/{}') format('truetype');
                            }}",
                            font.family(), fonts::output_filename(font)))
                        )
                    }
                  }
                }
                body class=(body_classes) {
                    div class=(header_classes) {
                        div class=(classes!("flex", "flex-col", "gap-x-2", "whitespace-nowrap"))
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
                                p class=(classes!("text-sm", "opacity-75")) { (DESCRIPTION) }
                            }

                        div class=(classes!("flex", "items-center", "gap-x-2")) {
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

                    div class=(classes!("flex", "justify-between", "flex-wrap", "items-end")) {
                        pre class=(classes!("text-xs")) { code { (*COMMIT_HASH) } }
                        a class=(classes!("text-sm")) href=(*REPO_URL) { "Source"}
                    }
                }
            }
        };
        markup
    }
}
