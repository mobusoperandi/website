use maud::Render;

use ssg_child::sources::BytesSource;
use ssg_child::sources::ExpectedFiles;
use ssg_child::FileSpec;

use crate::components::home_page::event_content_template;
use crate::expected_files::ExpectedFilesExt;
use crate::mob::Mob;
use crate::relative_path::RelativePathBuf;
use crate::{components, mob};

pub fn page(mobs: &[Mob]) -> FileSpec {
    let path = RelativePathBuf::from("/index.html");
    let mut expected_files = ExpectedFiles::default();

    let participants = mob::get_all_participants(mobs);

    let events = mobs
        .iter()
        .filter(|mob| {
            !matches!(
                mob.status(),
                mob::Status::Terminated(_) | mob::Status::Renamed(_)
            )
        })
        .map(|mob| mob.events(&mut expected_files, event_content_template))
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect();

    let base = components::PageBase::new(&mut expected_files, path.clone());

    let add_page_path = expected_files.insert_("/add.html");

    let home_page = components::home_page::HomePage::new(
        participants,
        mob::Status::legend(),
        events,
        base,
        add_page_path,
        expected_files.insert_("/fullcalendar.js"),
        expected_files.insert_("/rrule.js"),
        expected_files.insert_("/fullcalendar_rrule.js"),
    );

    let bytes = home_page.render().0.into_bytes();

    FileSpec::new(path, BytesSource::new(bytes, Some(expected_files)))
}
