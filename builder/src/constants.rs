use camino::Utf8PathBuf;
use once_cell::sync::Lazy;

use crate::url::Url;

pub(crate) const NAME: &str = "Mobus Operandi";
pub(crate) const DESCRIPTION: &str = "A mob programming community";

pub(crate) const MOBS_DIR: &str = "mobs";

pub(crate) static MOBS_PATH: Lazy<Utf8PathBuf> = Lazy::new(|| {
    [env!("CARGO_MANIFEST_DIR"), "..", MOBS_DIR]
        .iter()
        .collect()
});

pub(crate) static ZULIP_URL: Lazy<Url> =
    Lazy::new(|| "https://mobusoperandi.zulipchat.com".parse().unwrap());

pub(crate) const GITHUB_ORGANIZATION: &str = "mobusoperandi";

pub(crate) static GITHUB_ORGANIZATION_URL: Lazy<Url> = Lazy::new(|| {
    let mut url = Url::parse("https://github.com/").unwrap();

    url.set_path(GITHUB_ORGANIZATION);

    url
});

pub(crate) static COMMIT_HASH: Lazy<String> = Lazy::new(|| {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "exit code: {:?}",
        output.status.code()
    );

    String::from_utf8(output.stdout).unwrap()
});

const REPOSITORY: &str = "website";

pub(crate) static REPO_URL: Lazy<Url> = Lazy::new(|| {
    let mut repo_url = GITHUB_ORGANIZATION_URL.clone();
    repo_url.path_segments_mut().unwrap().push(REPOSITORY);
    repo_url
});

pub(crate) const DEFAULT_BRANCH: &str = "master";

pub(crate) const GITHUB_PULL_REQUESTS_URL: &str = "https://docs.github.com\
    /en/pull-requests/collaborating-with-pull-requests\
    /proposing-changes-to-your-work-with-pull-requests/about-pull-requests";
