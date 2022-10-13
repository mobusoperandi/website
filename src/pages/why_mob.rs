use super::base;
use maud::html;
use ssg::{Asset, Source};

pub fn page() -> Asset {
    Asset::new("why_mob.html".into(), async {
        Source::Bytes(
            base(
                html! {
                    p { "Because you'll learn and level-up on numerous skills:" }
                    ul {
                        li { "Communication" }
                        li { "Collaboration" }
                        li { "Knowledge sharing" }
                        li { "Various development tools and workflows" }
                    }

                    p { "You'll make friends." }

                    p { "You'll have fun." }

                    p { "You may build something you'll be proud of. " }
                },
                [],
                "".into(),
                "prose mx-auto".into(),
            )
            .0
            .into_bytes(),
        )
    })
}
