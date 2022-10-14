use maud::html;
use ssg::{Asset, Source};

use super::base;

pub fn page() -> Asset {
    Asset::new("mightyiam.html".into(), async {
        Source::Bytes(
            base(
                html! {
                    a href="https://github.com/mightyiam" { "Shahar Dawn Or (mightyiam)" }
                },
                [],
                "".into(),
            )
            .0
            .into_bytes(),
        )
    })
}
