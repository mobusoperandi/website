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
            &PathBuf::from("src/input.css").to_string_lossy(),
            "--output",
            &PathBuf::from(format!("./{OUTPUT_DIR}/index.css")).to_string_lossy(),
            "--content",
            // TODO explicit list instead of pattern
            &PathBuf::from(format!("./{OUTPUT_DIR}/**/*.html")).to_string_lossy(),
        ])
        .output()
        .await
        .unwrap();

    stdout().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}
