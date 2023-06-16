use maud::Render;

use ssg_child::sources::bytes::BytesSource;
use ssg_child::sources::ExpectedTargets;
use ssg_child::FileSpec;

use crate::components::home_page::event_content_template;
use crate::expected_targets::ExpectedTargetsExt;
use crate::mob::MOBS;
use crate::relative_path::RelativePathBuf;
use crate::{components, mob};

pub async fn page() -> FileSpec {
    let target_path = RelativePathBuf::from("/index.html");
    let mut expected_targets = ExpectedTargets::default();

    let participants = mob::get_all_participants();

    let events = MOBS
        .iter()
        .map(|mob| mob.events(&mut expected_targets, event_content_template))
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect();

    let base = components::PageBase::new(&mut expected_targets, target_path.to_owned());

    let add_page_path = expected_targets.insert_("/add.html");

    let home_page = components::home_page::HomePage::new(
        participants,
        mob::Status::legend(),
        events,
        base,
        add_page_path,
        expected_targets.insert_("/fullcalendar.js"),
        expected_targets.insert_("/rrule.js"),
        expected_targets.insert_("/fullcalendar_rrule.js"),
    );

    let bytes = home_page.render().0.into_bytes();

    FileSpec::new(target_path, BytesSource::new(bytes, Some(expected_targets)))
}
