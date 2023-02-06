use std::ffi::OsStr;

use anyhow::{bail, Result};
use once_cell::sync::Lazy;
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
