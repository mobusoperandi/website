use std::path::PathBuf;

pub(crate) const VOLLKORN: ssg::GoogleFont = ssg::GoogleFont {
    name: "Vollkorn",
    version: 21,
    subset: "latin",
    variant: "regular",
};

pub(crate) const ALL: [ssg::GoogleFont; 1] = [VOLLKORN];

pub(crate) fn assets() -> [ssg::Asset; 1] {
    ALL.map(|font| {
        ssg::Asset::new(
            PathBuf::from(format!("{}.ttf", font.name.to_lowercase())),
            async move { ssg::Source::GoogleFont(font) },
        )
    })
}

pub(crate) fn output_filename(font: &ssg::GoogleFont) -> String {
    format!("{}.ttf", font.name.to_lowercase())
}
