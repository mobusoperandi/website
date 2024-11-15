use camino::Utf8Path;
use itertools::Itertools;
use ssg_child::FileSpec;

use crate::{fonts, graphic_file_specs, pages};

pub(crate) fn get(mobs_path: &Utf8Path) -> impl Iterator<Item = FileSpec> {
    let fonts = fonts::all();
    let mobs = crate::mob::get_all(mobs_path).into_iter().collect_vec();
    let pages = pages::all(mobs);

    let calendar_library = FileSpec::new(
        "/fullcalendar.js",
        include_bytes!(env!("FULLCALENDAR")).as_slice(),
    );

    let rrule_library = FileSpec::new("/rrule.js", include_bytes!(env!("RRULE")).as_slice());

    let fullcalendar_rrule = FileSpec::new(
        "/fullcalendar_rrule.js",
        include_bytes!(env!("FULLCALENDAR_RRULE")).as_slice(),
    );

    [calendar_library, rrule_library, fullcalendar_rrule]
        .into_iter()
        .chain(fonts)
        .chain(graphic_file_specs::get())
        .chain(pages)
}
