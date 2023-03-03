use ssg::{FileSource, FileSpec};

use crate::{components, fonts, graphic_file_specs, pages};

pub(crate) async fn get() -> impl Iterator<Item = FileSpec> {
    let fonts = fonts::all();
    let pages = pages::all().await;

    let calendar_library = FileSpec::new(
        "/fullcalendar.js",
        FileSource::Http(components::CALENDAR_LIBRARY_URL.to_inner().clone()),
    );

    [calendar_library]
        .into_iter()
        .chain(fonts)
        .chain(graphic_file_specs::get())
        .chain(pages)
}
