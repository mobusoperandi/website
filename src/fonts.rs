use std::path::PathBuf;

use ssg::{Asset, GoogleFont, Source};

pub(crate) const VOLLKORN: GoogleFont = GoogleFont {
    name: "Vollkorn",
    version: 21,
    subset: "latin",
    variant: "regular",
};

pub(crate) const ALL: [GoogleFont; 1] = [VOLLKORN];

pub(crate) fn assets() -> [Asset; 1] {
    ALL.map(|font| {
        Asset::new(
            PathBuf::from(format!("{}.ttf", font.name.to_lowercase())),
            async move { Source::GoogleFont(font) },
        )
    })
}

pub(crate) fn output_filename(font: &GoogleFont) -> String {
    format!("{}.ttf", font.name.to_lowercase())
}
