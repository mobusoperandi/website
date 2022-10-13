use super::base;
use crate::environment::DEVELOPMENT;
use maud::html;
use ssg::{Asset, Source};

pub fn page() -> Asset {
    Asset::new("in_the_media.html".into(), async {
        Source::Bytes(base(
            html! {
                h1 { "In the media" }
                @if !DEVELOPMENT {
                    iframe
                        class="row-start-2 col-span-full"
                        width="560"
                        height="315"
                        src="https://www.youtube-nocookie.com/embed/nxNDo-7Fyfk"
                        title="YouTube video player"
                        frameborder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen {}
                }
            },
            [],
            "".into(),
            "".into()
            ).0.into_bytes())
    })
}
