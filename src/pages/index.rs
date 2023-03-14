use futures::FutureExt;
use maud::Render;
use ssg::{sources::FileSource, FileSpec};

use crate::{components, mobs};

pub async fn page() -> FileSpec {
    let mobs = mobs::read_all_mobs().await;
    let participants = mobs::get_all_participants().await;

    FileSpec::new(
        "/index.html",
        FileSource::BytesWithFileSpecSafety(Box::new(move |targets| {
            let mobs = mobs.clone();
            let participants = participants.clone();

            async {
                let base_page = components::BasePage {
                    title: None,
                    content: components::HomePage {
                        targets: targets.clone(),
                        mobs,
                        participants,
                    }
                    .render(),
                    content_classes: classes!("flex", "flex-col", "gap-1"),
                    targets,
                };

                Ok(base_page.render().0.into_bytes())
            }
            .boxed()
        })),
    )
}
