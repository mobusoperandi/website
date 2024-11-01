use std::io::{stdout, Write};

use camino::Utf8PathBuf;
use futures::{future::BoxFuture, FutureExt};
use tokio::process::Command;

use crate::OUTPUT_DIR;

pub(crate) fn execute(
) -> BoxFuture<'static, Result<(), Box<dyn std::error::Error + 'static + Send + Sync>>> {
    async {
        let output = Command::new("npx")
            .args([
                "tailwindcss",
                "--input",
                [env!("CARGO_MANIFEST_DIR"), "src", "input.css"]
                    .iter()
                    .collect::<Utf8PathBuf>()
                    .as_ref(),
                "--output",
                [".".as_ref(), OUTPUT_DIR.as_path(), "index.css".as_ref()]
                    .iter()
                    .collect::<Utf8PathBuf>()
                    .as_ref(),
                "--content",
                // TODO explicit list instead of pattern
                [
                    ".".as_ref(),
                    OUTPUT_DIR.as_path(),
                    "**".as_ref(),
                    "*.html".as_ref(),
                ]
                .iter()
                .collect::<Utf8PathBuf>()
                .as_ref(),
            ])
            .output()
            .await;

        let output = match output {
            Ok(output) => output,
            Err(error) => return Err(Box::new(error) as Box<dyn std::error::Error + Send + Sync>),
        };

        if let Err(error) = stdout().write_all(&output.stderr) {
            return Err(Box::new(error));
        }

        if !output.status.success() {
            return Err("tailwind failed".into());
        }

        Ok(())
    }
    .boxed()
}
