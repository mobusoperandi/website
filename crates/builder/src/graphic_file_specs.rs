use ssg_child::FileSpec;

pub(crate) fn get() -> [FileSpec; 5] {
    const FAVICON: [u8; 0] = [];
    let favicon = FileSpec::new("/favicon.ico", FAVICON.as_slice());
    let twitter_logo = FileSpec::new("/twitter_logo.svg", include_bytes!(env!("TWITTER_LOGO")));
    let zulip_logo = FileSpec::new("/zulip_logo.svg", include_bytes!(env!("ZULIP_LOGO")));
    let inverticat_logo = FileSpec::new("/inverticat.svg", include_bytes!(env!("INVERTICAT_LOGO")));
    let youtube_logo = FileSpec::new("/youtube_logo.svg", include_bytes!(env!("YOUTUBE_LOGO")));

    [
        favicon,
        twitter_logo,
        zulip_logo,
        inverticat_logo,
        youtube_logo,
    ]
}
