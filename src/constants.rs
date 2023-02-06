use once_cell::sync::Lazy;
use url::Url;

pub(crate) const NAME: &str = "Mobus Operandi";
pub(crate) const DESCRIPTION: &str = "A mob programming community";
pub(crate) const MOBS_PATH: &str = "mobs";
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

    if !output.status.success() {
        panic!("exit code: {:?}", output.status.code());
    };

    String::from_utf8(output.stdout).unwrap()
});

const REPOSITORY: &str = "website";

pub(crate) static REPO_URL: Lazy<Url> = Lazy::new(|| {
    let mut repo_url = GITHUB_ORGANIZATION_URL.clone();
    repo_url.path_segments_mut().unwrap().push(REPOSITORY);
    repo_url
});

pub(crate) const DEFAULT_BRANCH: &str = "master";

pub(crate) const OUTPUT_DIR: &str = ".vercel/output/static";
