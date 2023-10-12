#![warn(clippy::all, clippy::pedantic)]

mod dev;
use dev::app::state::BuilderState;
pub use dev::DevError;

#[derive(Debug)]
pub struct Parent {
    output_dir: camino::Utf8PathBuf,
    builder: dev::app::state::BuilderState,
}

impl Parent {
    pub fn new(output_dir: impl Into<camino::Utf8PathBuf>) -> Self {
        Self {
            output_dir: output_dir.into(),
            builder: BuilderState::default(),
        }
    }
}
