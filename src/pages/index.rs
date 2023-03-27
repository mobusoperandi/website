use std::convert::Infallible;

use crate::{components, constants::DESCRIPTION, mobs};

use futures::FutureExt;
use maud::Render;
use ssg::{sources::bytes_with_file_spec_safety::Targets, FileSpec};

pub async fn page() -> FileSpec {
    let mobs = mobs::read_all_mobs().await;
    let participants = mobs::get_all_participants().await;

    FileSpec::new("/index.html", move |targets: Targets| {
        let mobs = mobs.clone();
        let participants = participants.clone();

        async {
            let base_page = components::BasePage {
                title: None,
                content: components::HomePage {
                    targets: targets.clone(),
                    mobs,
                    participants,
                    status_legend: mobs::Status::legend(),
                }
                .render(),
                description: components::base_page::PageDescription::from(DESCRIPTION.to_owned()),
                content_classes: classes!("flex", "flex-col", "gap-1"),
                targets,
            };

            Ok::<_, Infallible>(base_page.render().0.into_bytes())
        }
        .boxed()
    })
}
