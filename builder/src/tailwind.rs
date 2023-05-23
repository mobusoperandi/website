use std::{
    io::{stdout, Write},
    path::PathBuf,
};

use anyhow::ensure;
use tokio::process::Command;

use crate::OUTPUT_DIR;

pub(crate) async fn execute() -> anyhow::Result<()> {
    let output = Command::new("npx")
        .args([
            "tailwindcss",
            "--input",
            &["src", "input.css"]
                .iter()
                .collect::<PathBuf>()
                .to_string_lossy(),
            "--output",
            &[".".as_ref(), OUTPUT_DIR.as_path(), "index.css".as_ref()]
                .iter()
                .collect::<PathBuf>()
                .to_string_lossy(),
            "--content",
            // TODO explicit list instead of pattern
            &[
                ".".as_ref(),
                OUTPUT_DIR.as_path(),
                "**".as_ref(),
                "*.html".as_ref(),
            ]
            .iter()
            .collect::<PathBuf>()
            .to_string_lossy(),
        ])
        .output()
        .await?;

    stdout().write_all(&output.stderr)?;

    ensure!(output.status.success());
    Ok(())
}
