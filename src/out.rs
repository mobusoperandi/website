use std::path;

#[derive(Clone)]
pub(crate) struct File {
    pub(crate) target_path: path::PathBuf,
    pub(crate) source: Source,
}

#[derive(Clone)]
pub(crate) enum Source {
    Markup(maud::Markup),
}

impl Source {
    pub(crate) fn into_string(self) -> String {
        match self {
            Source::Markup(markup) => markup.into_string(),
        }
    }
}
