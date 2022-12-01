mod environment;
mod fonts;
mod mobs;
mod pages;
use environment::OUTPUT_DIR;
use futures::{stream, StreamExt};
use once_cell::sync::Lazy;
use ssg::{generate_static_site, Asset, Source};
use std::{
    collections::BTreeSet,
    io::{stdout, Write},
    path::PathBuf,
};
use tokio::process::Command;
use url::Url;

pub(crate) const COPYRIGHT_HOLDER: &str = "Shahar Or";
pub(crate) const NAME: &str = "Mobus Operandi";
pub(crate) const MOBS_PATH: &str = "mobs";
pub(crate) static REPO_URL: Lazy<Url> = Lazy::new(|| {
    String::from_utf8(
        std::process::Command::new("gh")
            .args(["repo", "view", "--json", "url", "--jq", ".url"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .parse()
    .unwrap()
});
pub(crate) static DEFAULT_BRANCH: Lazy<String> = Lazy::new(|| {
    String::from_utf8(
        std::process::Command::new("gh")
            .args([
                "repo",
                "view",
                "--json",
                "defaultBranchRef",
                "--jq",
                ".defaultBranchRef.name",
            ])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
});

#[tokio::main]
async fn main() {
    //console_subscriber::init();
    let fonts = fonts::assets();
    let pages = pages::all().await;
    let favicon = Asset::new(PathBuf::from("favicon.ico"), async {
        Source::Bytes(vec![])
    });
    let fullcalendar_css = Asset::new(PathBuf::from("fullcalendar.css"), async {
        Source::Http(
            Url::parse("https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.css").unwrap(),
        )
    });
    let fullcalendar_js = Asset::new(PathBuf::from("fullcalendar.js"), async {
        Source::Http(
            Url::parse("https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.js").unwrap(),
        )
    });
    let files: BTreeSet<Asset> = [favicon, fullcalendar_css, fullcalendar_js]
        .into_iter()
        .chain(fonts)
        .chain(pages)
        .collect();
    // TODO exit code
    let generated = stream::iter(generate_static_site(OUTPUT_DIR.parse().unwrap(), files).unwrap())
        .map(|(path, source)| (path, tokio::spawn(source)))
        .for_each_concurrent(usize::MAX, |(path, join_handle)| async move {
            match join_handle.await.unwrap() {
                Ok(()) => {
                    println!("generated: {:?}", path);
                }
                // TODO know which file errored
                Err(error) => {
                    println!("error: {:?}", error);
                }
            }
        });
    tokio::join!(generated);
    produce_css().await;
}

async fn produce_css() {
    let output = Command::new("npx")
        .args([
            "tailwindcss",
            "--input",
            &PathBuf::from("src/input.css").to_string_lossy(),
            "--output",
            &PathBuf::from(format!("./{OUTPUT_DIR}/index.css")).to_string_lossy(),
            "--content",
            // TODO explicit list instead of pattern
            &PathBuf::from(format!("./{OUTPUT_DIR}/*.html")).to_string_lossy(),
        ])
        .output()
        .await
        .unwrap();
    stdout().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}
