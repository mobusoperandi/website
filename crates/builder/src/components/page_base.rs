use std::fmt::Display;

use chrono::Utc;
use maud::{html, Markup, Render, DOCTYPE};
use ssg_child::sources::ExpectedFiles;

use crate::{
    constants::{COMMIT_HASH, DESCRIPTION, GITHUB_ORGANIZATION_URL, NAME, REPO_URL, ZULIP_URL},
    expected_files::ExpectedFilesExt,
    fonts,
    html::Classes,
    relative_path::RelativePathBuf,
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

#[derive(Debug, Clone)]
pub(crate) struct PageBase {
    index: RelativePathBuf,
    current: RelativePathBuf,
    zulip_logo: RelativePathBuf,
    inverticat: RelativePathBuf,
    twitter_logo: RelativePathBuf,
}

impl PageBase {
    pub(crate) fn new(expected_files: &mut ExpectedFiles, current_path: RelativePathBuf) -> Self {
        Self {
            index: expected_files.insert_("/index.html"),
            current: current_path,
            zulip_logo: expected_files.insert_("/zulip_logo.svg"),
            inverticat: expected_files.insert_("/inverticat.svg"),
            twitter_logo: expected_files.insert_("/twitter_logo.svg"),
        }
    }

    pub(crate) fn into_page(
        self,
        title: Option<PageTitle>,
        head_content: Option<Markup>,
        content: Markup,
        content_classes: Classes,
        description: PageDescription,
    ) -> Page {
        Page {
            base: self,
            title,
            head_content,
            content,
            content_classes,
            description,
        }
    }
}

pub(crate) struct Page {
    base: PageBase,
    title: Option<PageTitle>,
    head_content: Option<Markup>,
    content: Markup,
    content_classes: Classes,
    description: PageDescription,
}

#[allow(clippy::too_many_lines)]
impl Render for Page {
    fn render(&self) -> Markup {
        const NAV_ICON_SIZE: u8 = 32;
        let version = Utc::now().timestamp_millis();

        let title = self
            .title
            .clone()
            .map_or(NAME.to_owned(), |title| format!("{title}; {NAME}"));

        let brand_classes = classes!("tracking-widest", "text-center");

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
                  @if let Some(head_content) = &self.head_content {
                      (head_content)
                  }
                  link rel="stylesheet" href={ "/index.css?v=" (version) };
                  style {
                    @for font in fonts::ALL.as_slice() { (font) }
                  }
                }
                body class=(body_classes) {
                    div class=(header_classes) {
                        div class=(classes!("flex", "flex-col", "gap-x-2", "whitespace-nowrap"))
                            {
                                @if self.base.index == self.base.current {
                                    p
                                        class=(brand_classes)
                                        { (NAME) }
                                } @else {
                                    a
                                        href=(self.base.index)
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
                                    src=(self.base.zulip_logo);
                            }

                            a class=(classes!("invert")) href=(*GITHUB_ORGANIZATION_URL) {
                                img
                                    width=(NAV_ICON_SIZE)
                                    alt="GitHub"
                                    src=(self.base.inverticat);
                            }

                            a href="https://twitter.com/mobusoperandi" {
                                img
                                    width=(NAV_ICON_SIZE)
                                    alt="Twitter"
                                    src=(self.base.twitter_logo);
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
