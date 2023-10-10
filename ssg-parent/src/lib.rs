#![warn(clippy::all, clippy::pedantic)]

mod dev;

pub use dev::DevError;

pub struct Parent {
    output_dir: camino::Utf8PathBuf,
}

#[derive(Debug,)]
enum BuildError 

impl Parent {
    #[must_use]
    pub fn new(output_dir: impl Into<camino::Utf8PathBuf>) -> Self {
        Self {
            output_dir: output_dir.into(),
        }
    }

    pub fn build(&self) -> Result<(), BuildError> {

    }
}
