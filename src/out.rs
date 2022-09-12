use std::path;

use crate::fonts;

#[derive(Clone)]
pub(crate) struct File {
    pub(crate) target_path: path::PathBuf,
    pub(crate) source: Source,
}

#[derive(Clone)]
pub(crate) enum Source {
    Markup(maud::Markup),
    Font(fonts::Font),
    Bytes(Vec<u8>),
}
