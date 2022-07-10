use maud::html;

use super::Section;

pub fn section() -> Section {
    Section::new(
        "mightyiam".into(),
        "".into(),
        None,
        html! {
            a href="https://github.com/mightyiam" { "Shahar Dawn Or (mightyiam)" }
        },
    )
}
