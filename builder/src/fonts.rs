use once_cell::sync::Lazy;
use ssg_child::{sources::GoogleFont, FileSpec};

use crate::relative_path::RelativePathBuf;

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

pub(crate) fn output_filename(font: &GoogleFont) -> RelativePathBuf {
    format!("{}.ttf", font.family().to_lowercase()).into()
}
