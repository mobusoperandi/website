#![warn(clippy::all, clippy::pedantic)]

mod dev;

use std::{
    num::{NonZeroI16, NonZeroI32},
    process::Stdio,
};

pub use dev::DevError;
use futures::{StreamExt, TryStreamExt};
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
    ExitCode(NonZeroI32),
    #[error("")]
    ExitWithoutCode,
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

    pub async fn build(&self) -> (Result<(), BuildError>, String) {
        let builder_command = &mut self.builder_command();
        builder_command.stdout(Stdio::null());
        builder_command.stderr(Stdio::piped());

        let mut child = builder_command
            .spawn()
            .map_err(|error| (error.into(), String::new()))?;
        let child_stderr = child.stderr.take().expect("stderr should be piped");

        let child_stderr =
            tokio_stream::wrappers::LinesStream::new(BufReader::new(child_stderr).lines());

        let child_stderr: String = child_stderr
            .inspect_ok(|line| eprintln!("builder: {line}"))
            .try_collect()
            .await
            .map_err(|error| (error.into(), String::new()))?;

        match child.wait().await {
            Err(error) => Err((error.into(), child_stderr)),
            Ok(exit_status) => match exit_status.code() {
                Some(0) => Ok(child_stderr),
                None => Err((BuildError::ExitWithoutCode, child_stderr)),
                Some(non_zero) => Err((
                    BuildError::ExitCode(NonZeroI32::new(non_zero).unwrap()),
                    child_stderr,
                )),
            },
        }
    }
}
