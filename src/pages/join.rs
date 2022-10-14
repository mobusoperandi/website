use maud::html;
use ssg::{Asset, Source};

use super::base;

pub fn page() -> Asset {
    Asset::new("join.html".into(), async {
        Source::Bytes(
            base(
                html! {
                    p {
                        "Join by "
                        a href="https://calendly.com/mightyiam" {
                            "scheduling a chat with Dawn"
                        }
                    }
                },
                [],
                "prose mx-auto".into(),
            )
            .0
            .into_bytes(),
        )
    })
}
