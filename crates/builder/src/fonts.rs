use once_cell::sync::Lazy;
use ssg_child::FileSpec;

use crate::google_font::TrueTypeFont;

pub(crate) const VOLLKORN: TrueTypeFont =
    TrueTypeFont::new(include_bytes!(env!("VOLLKORN")), "Vollkorn");

pub(crate) static ALL: Lazy<[TrueTypeFont; 1]> = Lazy::new(|| [VOLLKORN.clone()]);

pub(crate) fn all() -> [FileSpec; 1] {
    ALL.clone()
        .map(|font| FileSpec::new(format!("/{}.ttf", font.family().to_lowercase()), font))
}
