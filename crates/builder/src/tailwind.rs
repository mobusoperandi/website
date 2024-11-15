use std::io::{stdout, Write};

use camino::{Utf8Path, Utf8PathBuf};
use tempfile::NamedTempFile;
use tokio::process::Command;

pub(crate) async fn execute(output_dir: &Utf8Path) {
    let input_contents = include_bytes!(env!("TAILWINDCSS_INPUT"));
    let mut input_file = NamedTempFile::new().unwrap();
    input_file.write_all(input_contents).unwrap();

    let output = Command::new(env!("TAILWINDCSS"))
        .args([
            "--config",
            env!("TAILWINDCSS_CONFIG"),
            "--input",
            input_file.path().to_str().unwrap(),
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
        .await
        .unwrap();

    stdout().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}
