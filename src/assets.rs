use ssg::{Asset, Source};

use crate::{components, fonts, graphic_assets, pages};

pub(crate) async fn get() -> impl Iterator<Item = Asset> {
    let fonts = fonts::assets();
    let pages = pages::all().await;

    let calendar_library = Asset::new("/fullcalendar.js", async {
        Source::Http(components::CALENDAR_LIBRARY_URL.to_inner().clone())
    });

    [calendar_library]
        .into_iter()
        .chain(fonts)
        .chain(graphic_assets::get())
        .chain(pages)
}
