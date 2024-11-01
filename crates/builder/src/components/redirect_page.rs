use maud::{html, Render};

use super::PageBase;

#[derive(Debug, Clone)]
pub(crate) struct RedirectPage {
    base: PageBase,
    target: String,
}

impl RedirectPage {
    pub(crate) fn new(base: PageBase, target: String) -> Self {
        Self { base, target }
    }
}

impl Render for RedirectPage {
    fn render(&self) -> maud::Markup {
        let title = Some("Redirecting...".to_owned().into());

        let head_content = Some(html! {
            meta http-equiv="refresh" content=(format!("5; url={}", self.target));
        });

        let content = html! {
            p {
                "Redirecting to";
                code { (self.target) };
                "...";
            }
        };

        let content_classes = classes!("flex", "justify-center", "items-center");

        let description = format!("Redirect to {}...", self.target).to_owned().into();

        let page =
            self.base
                .clone()
                .into_page(title, head_content, content, content_classes, description);

        page.render()
    }
}
