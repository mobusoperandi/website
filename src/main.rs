mod environment;
mod fonts;
#[macro_use]
mod html;
mod assets;
mod calendar;
mod graphic_assets;
mod markdown;
mod mobs;
mod pages;
mod style;
use anyhow::{bail, Result};
use environment::OUTPUT_DIR;
use futures::{stream, StreamExt};
use once_cell::sync::Lazy;
use ssg::generate_static_site;
use std::{
    ffi::OsStr,
    io::{stdout, Write},
    path::PathBuf,
};
use tokio::process::Command;
use url::Url;

pub(crate) const NAME: &str = "Mobus Operandi";
pub(crate) const DESCRIPTION: &str = "A mob programming community";
pub(crate) const MOBS_PATH: &str = "mobs";
pub(crate) static ZULIP_URL: Lazy<Url> =
    Lazy::new(|| "https://mobusoperandi.zulipchat.com".parse().unwrap());

pub(crate) static GITHUB_ORGANIZATION: Lazy<String> = Lazy::new(|| {
    string_from_command(
        "gh",
        ["repo", "view", "--json", "owner", "--jq", ".owner.login"],
    )
    .unwrap()
    .trim_end()
    .to_owned()
});

pub(crate) static GITHUB_ORGANIZATION_URL: Lazy<Url> = Lazy::new(|| {
    let mut url = Url::parse("https://github.com/").unwrap();
    url.set_path(GITHUB_ORGANIZATION.as_str());
    url
});

pub(crate) static COMMIT_HASH: Lazy<String> =
    Lazy::new(|| string_from_command("git", ["rev-parse", "HEAD"]).unwrap());

fn string_from_command<I: AsRef<OsStr>>(
    program: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = I>,
) -> Result<String> {
    let output = std::process::Command::new(program).args(args).output()?;
    if !output.status.success() {
        bail!("exit code: {:?}", output.status.code());
    };
    let output = String::from_utf8(output.stdout)?;
    Ok(output)
}

pub(crate) static REPO_URL: Lazy<Url> = Lazy::new(|| {
    string_from_command("gh", ["repo", "view", "--json", "url", "--jq", ".url"])
        .unwrap()
        .parse()
        .unwrap()
});

pub(crate) static DEFAULT_BRANCH: Lazy<String> = Lazy::new(|| {
    string_from_command(
        "gh",
        [
            "repo",
            "view",
            "--json",
            "defaultBranchRef",
            "--jq",
            ".defaultBranchRef.name",
        ],
    )
    .unwrap()
});

#[tokio::main]
async fn main() {
    // TODO exit code
    let assets = assets::get().await;
    let generated =
        stream::iter(generate_static_site(OUTPUT_DIR.parse().unwrap(), assets).unwrap())
            .map(|(path, source)| (path, tokio::spawn(source)))
            .for_each_concurrent(usize::MAX, |(path, join_handle)| async move {
                println!("generating: {path:?}");
                join_handle
                    .await
                    .unwrap()
                    .unwrap_or_else(|error| panic!("{path:?}: {error:?}"));
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
            &PathBuf::from(format!("./{OUTPUT_DIR}/**/*.html")).to_string_lossy(),
        ])
        .output()
        .await
        .unwrap();
    stdout().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}
