use once_cell::sync::Lazy;
use ssg_child::FileSpec;

use crate::google_font::GoogleFont;

pub(crate) static VOLLKORN: Lazy<GoogleFont> = Lazy::new(|| {
    GoogleFont::new(
        "Vollkorn".to_owned(),
        "latin".to_owned(),
        "regular".to_owned(),
    )
});

pub(crate) static ALL: Lazy<[GoogleFont; 1]> = Lazy::new(|| [VOLLKORN.clone()]);

pub(crate) fn all() -> [FileSpec; 1] {
    ALL.clone()
        .map(|font| FileSpec::new(format!("/{}.ttf", font.family().to_lowercase()), font))
}
