use std::path::PathBuf;

use ssg::{Asset, Source};
use url::Url;

pub(crate) fn get() -> [Asset; 5] {
    let favicon = Asset::new(PathBuf::from("favicon.ico"), async {
        Source::Bytes(vec![])
    });

    let twitter_logo = Asset::new(PathBuf::from("twitter_logo.svg"), async {
        Source::Http(
            Url::parse("https://upload.wikimedia.org/wikipedia/commons/4/4f/Twitter-logo.svg")
                .unwrap(),
        )
    });

    let zulip_logo = Asset::new(PathBuf::from("zulip_logo.svg"), async {
        Source::Http(
            Url::parse("https://raw.githubusercontent.com/zulip/zulip/main/static/images/logo/zulip-icon-square.svg")
                .unwrap(),
        )
    });

    let inverticat_logo = Asset::new(PathBuf::from("inverticat.svg"), async {
        Source::Http(
            Url::parse(
                "https://upload.wikimedia.org/wikipedia/commons/9/91/Octicons-mark-github.svg",
            )
            .unwrap(),
        )
    });

    let youtube_logo = Asset::new(PathBuf::from("youtube_logo.svg"), async {
        Source::Http(
            Url::parse("https://upload.wikimedia.org/wikipedia/commons/0/09/YouTube_full-color_icon_%282017%29.svg")
                .unwrap(),
        )
    });

    [
        favicon,
        twitter_logo,
        zulip_logo,
        inverticat_logo,
        youtube_logo,
    ]
}
