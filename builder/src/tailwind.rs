use std::io::{stdout, Write};

use anyhow::ensure;
use camino::{Utf8Path, Utf8PathBuf};
use tokio::process::Command;

pub(crate) async fn execute(output_dir: &Utf8Path) -> anyhow::Result<()> {
    let output = Command::new("npx")
        .args([
            "tailwindcss",
            "--input",
            [env!("CARGO_MANIFEST_DIR"), "src", "input.css"]
                .iter()
                .collect::<Utf8PathBuf>()
                .as_ref(),
            "--output",
            [".".as_ref(), output_dir, "index.css".as_ref()]
                .iter()
                .collect::<Utf8PathBuf>()
                .as_ref(),
            "--content",
            // TODO explicit list instead of pattern
            [".".as_ref(), output_dir, "**".as_ref(), "*.html".as_ref()]
                .iter()
                .collect::<Utf8PathBuf>()
                .as_ref(),
        ])
        .output()
        .await?;

    stdout().write_all(&output.stderr)?;

    ensure!(output.status.success());
    Ok(())
}
