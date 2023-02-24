use std::{
    io::{stdout, Write},
    path::PathBuf,
};

use tokio::process::Command;

use crate::OUTPUT_DIR;

pub(crate) async fn execute() {
    let output = Command::new("npx")
        .args([
            "tailwindcss",
            "--input",
            &["src", "input.css"]
                .iter()
                .collect::<PathBuf>()
                .to_string_lossy(),
            "--output",
            &[".", OUTPUT_DIR, "index.css"]
                .iter()
                .collect::<PathBuf>()
                .to_string_lossy(),
            "--content",
            // TODO explicit list instead of pattern
            &[".", OUTPUT_DIR, "**", "*.html"]
                .iter()
                .collect::<PathBuf>()
                .to_string_lossy(),
        ])
        .output()
        .await
        .unwrap();

    stdout().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}
