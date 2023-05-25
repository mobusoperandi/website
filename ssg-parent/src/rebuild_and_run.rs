use thiserror::Error;
use tokio::process::Command;

#[derive(Error, Debug)]
pub enum WatchError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Exit(std::process::ExitStatus),
}

pub async fn watch_for_changes_and_rebuild() -> WatchError {
    let child = Command::new("cargo")
        .args([
            "bin",
            "cargo-watch",
            "--workdir",
            "builder",
            "--exec",
            "run",
        ])
        .spawn();

    let mut child = match child {
        Ok(child) => child,
        Err(err) => return err.into(),
    };

    // success case is indefinitely waiting here
    let status = match child.wait().await {
        Ok(status) => status,
        Err(err) => return err.into(),
    };

    WatchError::Exit(status)
}
