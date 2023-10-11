#![warn(clippy::all, clippy::pedantic)]

mod dev;

use std::{num::NonZeroI16, process::Stdio};

pub use dev::DevError;
use tokio::io::{AsyncBufReadExt, BufReader};

const BUILDER_CRATE_NAME: &str = "builder";

pub struct Parent {
    output_dir: camino::Utf8PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("")]
    ExitCode(NonZeroI16),
}

impl Parent {
    #[must_use]
    pub fn new(output_dir: impl Into<camino::Utf8PathBuf>) -> Self {
        Self {
            output_dir: output_dir.into(),
        }
    }

    fn builder_command(&self) -> tokio::process::Command {
        let mut command = tokio::process::Command::new("cargo");

        command.args([
            "run",
            "--package",
            BUILDER_CRATE_NAME,
            "--",
            self.output_dir.as_str(),
        ]);

        command
    }

    pub async fn build(&self) -> Result<(), BuildError> {
        let builder_command = &mut self.builder_command();
        builder_command.stdout(Stdio::null());
        builder_command.stderr(Stdio::piped());
        let mut child = builder_command.spawn()?;
        let stderr = child.stderr.take().expect("stderr should be piped");
        let stderr = tokio_stream::wrappers::LinesStream::new(BufReader::new(stderr).lines());
        let stderr = stderr.map
        todo!()
    }
}
