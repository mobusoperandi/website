use maud::html;

use super::Section;

pub fn section() -> Section {
    Section::new(
        "why_mob".into(),
        "".into(),
        None,
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
    )
}
