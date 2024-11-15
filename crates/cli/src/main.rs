#![warn(clippy::all, clippy::pedantic)]

use camino::Utf8PathBuf;
use clap::Parser;
use ssg_parent::Parent;

#[derive(Debug, Parser)]
struct Cli {
    mobs_path: Utf8PathBuf,
    output_dir: Utf8PathBuf,
    /// open website in a browser
    #[arg(short, long)]
    open: bool,
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "tokio_console")]
    console_subscriber::init();

    let cli = Cli::parse();

    let parent = Parent::new(
        &cli.output_dir,
        "cargo",
        [
            "run",
            "--package",
            "builder",
            "--",
            cli.mobs_path.as_str(),
            cli.output_dir.as_str(),
        ],
    );

    let error = parent
        .dev(
            [Utf8PathBuf::from_iter([
                env!("CARGO_MANIFEST_DIR"),
                "..",
                "builder",
            ])],
            cli.open,
        )
        .await;
    panic!("{error}");
}
