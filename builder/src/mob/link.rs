use maud::{html, Render};
use schema::Schema;
use serde::{Deserialize, Serialize};
use ssg_child::sources::ExpectedTargets;

use crate::{expected_targets::ExpectedTargetsExt, relative_path::RelativePathBuf, url::Url};

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
/// A link that showcases the mob
pub(crate) enum Link {
    /// A YouTube channel ID
    ///
    /// Example:
    ///
    /// ```yaml
    /// !YouTube "@mobseattle"
    /// ```
    YouTube(String),
}

impl From<(Link, &mut ExpectedTargets)> for LinkElement {
    fn from((link, expected_targets): (Link, &mut ExpectedTargets)) -> Self {
        let (url, image_path, alt) = match link {
            Link::YouTube(path) => {
                let url = Url::parse(&format!("https://www.youtube.com/{path}")).unwrap();
                let image_path = expected_targets.insert_("/youtube_logo.svg");
                let alt = "YouTube";
                (url, image_path, alt)
            }
        };

        LinkElement::new(url, alt, image_path)
    }
}

pub(crate) struct LinkElement {
    url: Url,
    alt: &'static str,
    image_path: RelativePathBuf,
}

impl LinkElement {
    fn new(url: Url, alt: &'static str, image_path: RelativePathBuf) -> Self {
        Self {
            url,
            alt,
            image_path,
        }
    }
}

impl Render for LinkElement {
    fn render(&self) -> maud::Markup {
        html! {
            a href=(self.url) {
                img
                    class=(classes!("h-8"))
                    alt=(self.alt)
                    src=(self.image_path);
            }
        }
    }
}
