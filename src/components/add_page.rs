use maud::{html, Render};
use ssg::sources::bytes_with_file_spec_safety::Targets;

use crate::components;
use crate::constants::{DEFAULT_BRANCH, GITHUB_PULL_REQUESTS_URL, MOBS_PATH, REPO_URL};
use crate::style::{PROSE_CLASSES, VERTICAL_GAP_CLASS};

use super::schema::type_::Type;

pub(crate) struct AddPage {
    internal_types: Vec<Type>,
    targets: Targets,
}

impl AddPage {
    pub(crate) fn new(internal_types: Vec<Type>, targets: Targets) -> Self {
        Self {
            internal_types,
            targets,
        }
    }
}

impl Render for AddPage {
    fn render(&self) -> maud::Markup {
        let mut existing_mobs_url = REPO_URL.clone();

        existing_mobs_url
            .path_segments_mut()
            .unwrap()
            .push("tree")
            .push(DEFAULT_BRANCH)
            .push(MOBS_PATH);

        let content = html! {
            div class=(*PROSE_CLASSES) {
                h1 { "Add a mob" }
                p {
                    "Mobs are specified in files in "
                    a href=(existing_mobs_url) {
                        "the "
                        code { (MOBS_PATH) }
                        " directory."
                    }
                    " Take a look at some of those files for examples."
                }
                p {
                    "To add a mob, submit a "
                    a href=(GITHUB_PULL_REQUESTS_URL) { "pull request" }
                    " that adds a mob file."
                }
            }
            ol class=(classes!("flex", "flex-col", VERTICAL_GAP_CLASS)) {
                @for type_ in &self.internal_types {
                    li { (type_) }
                }
            }
        };

        components::BasePage {
            title: Some("Add".to_owned().into()),
            content,
            content_classes: classes!("flex", "flex-col", VERTICAL_GAP_CLASS),
            targets: self.targets.clone(),
        }
        .render()
    }
}
