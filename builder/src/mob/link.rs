use maud::{html, Render};
use schema::Schema;
use serde::{Deserialize, Serialize};
use ssg_child::sources::bytes_with_file_spec_safety::{TargetNotFoundError, Targets};

use crate::url::Url;

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

impl TryFrom<(Link, &Targets)> for LinkElement {
    type Error = TargetNotFoundError;

    fn try_from((link, targets): (Link, &Targets)) -> Result<Self, Self::Error> {
        let (url, image_path, alt) = match link {
            Link::YouTube(path) => {
                let url = Url::parse(&format!("https://www.youtube.com/{path}")).unwrap();
                let image_path = targets.path_of("/youtube_logo.svg")?;
                let alt = "YouTube";
                (url, image_path, alt)
            }
        };

        Ok(LinkElement::new(url, alt, image_path))
    }
}

pub(crate) struct LinkElement {
    url: Url,
    alt: &'static str,
    image_path: String,
}

impl LinkElement {
    fn new(url: Url, alt: &'static str, image_path: String) -> Self {
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
