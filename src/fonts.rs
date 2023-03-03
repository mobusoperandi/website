use ssg::{FileSource, FileSpec, GoogleFont};

pub(crate) const VOLLKORN: GoogleFont = GoogleFont {
    name: "Vollkorn",
    version: 21,
    subset: "latin",
    variant: "regular",
};

pub(crate) const ALL: [GoogleFont; 1] = [VOLLKORN];

pub(crate) fn all() -> [FileSpec; 1] {
    ALL.map(|font| {
        FileSpec::new(
            format!("/{}.ttf", font.name.to_lowercase()),
            FileSource::GoogleFont(font),
        )
    })
}

pub(crate) fn output_filename(font: &GoogleFont) -> String {
    format!("{}.ttf", font.name.to_lowercase())
}
