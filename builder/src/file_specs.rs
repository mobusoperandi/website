use anyhow::Result;
use ssg_child::FileSpec;

use crate::{components, fonts, graphic_file_specs, pages};

pub(crate) async fn get() -> Result<impl Iterator<Item = FileSpec>> {
    let fonts = fonts::all();
    let pages = pages::all().await?;

    let calendar_library = FileSpec::new(
        "/fullcalendar.js",
        ssg_child::sources::Http::from(components::CALENDAR_LIBRARY_URL.to_inner().clone()),
    );

    let rrule_library = FileSpec::new(
        "/rrule.js",
        ssg_child::sources::Http::from(components::CALENDAR_RRULE_URL.to_inner().clone()),
    );

    let fullcalendar_rrule = FileSpec::new(
        "/fullcalendar_rrule.js",
        ssg_child::sources::Http::from(
            components::CALENDAR_FULLCALENDAR_RRULE_URL
                .to_inner()
                .clone(),
        ),
    );

    Ok([calendar_library, rrule_library, fullcalendar_rrule]
        .into_iter()
        .chain(fonts)
        .chain(graphic_file_specs::get())
        .chain(pages))
}
