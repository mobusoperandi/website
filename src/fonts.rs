use once_cell::sync::Lazy;
use ssg::{sources::GoogleFont, FileSpec};

pub(crate) static VOLLKORN: Lazy<GoogleFont> = Lazy::new(|| {
    GoogleFont::new(
        "Vollkorn".to_owned(),
        21,
        "latin".to_owned(),
        "regular".to_owned(),
    )
});

pub(crate) static ALL: Lazy<[GoogleFont; 1]> = Lazy::new(|| [VOLLKORN.clone()]);

pub(crate) fn all() -> [FileSpec; 1] {
    ALL.clone()
        .map(|font| FileSpec::new(format!("/{}.ttf", font.family().to_lowercase()), font))
}

pub(crate) fn output_filename(font: &GoogleFont) -> String {
    format!("{}.ttf", font.family().to_lowercase())
}
