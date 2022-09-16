use std::path;

use crate::{environment::OUTPUT_DIR, ssg};

pub(crate) const VOLLKORN: ssg::GoogleFont = ssg::GoogleFont {
    name: "Vollkorn",
    version: 21,
    subset: "latin",
    variant: "regular",
};

pub(crate) const ALL: [ssg::GoogleFont; 1] = [VOLLKORN];

pub(crate) fn ssg_inputs() -> [ssg::Input; 1] {
    ALL.map(|font| ssg::Input {
        target_path: path::PathBuf::from(OUTPUT_DIR)
            .join(format!("{}.ttf", font.name.to_lowercase())),
        source: ssg::Source::GoogleFont(font),
    })
}

pub(crate) fn output_filename(font: &ssg::GoogleFont) -> String {
    format!("{}.ttf", font.name.to_lowercase())
}
