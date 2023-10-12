#![warn(clippy::all, clippy::pedantic)]

mod dev;
pub use dev::DevError;

pub struct Parent {
    output_dir: camino::Utf8PathBuf,
}

impl Parent {
    pub fn new(output_dir: impl Into<camino::Utf8PathBuf>) -> Self {
        Self {
            output_dir: output_dir.into(),
        }
    }
}
