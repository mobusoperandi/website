use ssg::FileSpec;

use crate::url::Url;

pub(crate) fn get() -> [FileSpec; 5] {
    const FAVICON: [u8; 0] = [];
    let favicon = FileSpec::new("/favicon.ico", FAVICON.as_slice());

    let twitter_logo = FileSpec::new(
        "/twitter_logo.svg",
        ssg::sources::Http::from(
            Url::parse("https://upload.wikimedia.org/wikipedia/commons/6/6f/Logo_of_Twitter.svg")
                .unwrap()
                .to_inner()
                .clone(),
        ),
    );

    let zulip_logo = FileSpec::new("/zulip_logo.svg", 
        ssg::sources::Http::from(
            Url::parse("https://raw.githubusercontent.com/zulip/zulip/main/static/images/logo/zulip-icon-square.svg")
                .unwrap().to_inner().clone(),
        )
    );

    let inverticat_logo = FileSpec::new(
        "/inverticat.svg",
        ssg::sources::Http::from(
            Url::parse(
                "https://upload.wikimedia.org/wikipedia/commons/c/c2/GitHub_Invertocat_Logo.svg",
            )
            .unwrap()
            .to_inner()
            .clone(),
        ),
    );

    let youtube_logo = FileSpec::new("/youtube_logo.svg",
        ssg::sources::Http::from(
            Url::parse("https://upload.wikimedia.org/wikipedia/commons/0/09/YouTube_full-color_icon_%282017%29.svg")
                .unwrap().to_inner().clone(),
        )
    );

    [
        favicon,
        twitter_logo,
        zulip_logo,
        inverticat_logo,
        youtube_logo,
    ]
}
