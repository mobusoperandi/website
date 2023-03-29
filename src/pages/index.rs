use futures::FutureExt;
use maud::Render;

use ssg::sources::bytes_with_file_spec_safety::TargetNotFoundError;
use ssg::{sources::bytes_with_file_spec_safety::Targets, FileSpec};

use crate::components::home_page::event_content_template;
use crate::{components, mobs};

pub async fn page() -> FileSpec {
    let mobs = mobs::read_all_mobs().await;
    let participants = mobs::get_all_participants().await;

    FileSpec::new("/index.html", move |targets: Targets| {
        let mobs = mobs.clone();
        let participants = participants.clone();

        async move {
            let events = mobs
                .iter()
                .map(|mob| mob.events(&targets, event_content_template))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect();

            let base = components::PageBase::new(targets.clone())?;

            let home_page = components::home_page::HomePage {
                targets: targets.clone(),
                participants,
                status_legend: mobs::Status::legend(),
                events,
                base,
            };

            Ok::<_, TargetNotFoundError>(home_page.render().0.into_bytes())
        }
        .boxed()
    })
}
