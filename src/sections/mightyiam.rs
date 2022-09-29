use maud::html;

use super::Section;

pub fn section() -> Section {
    Section {
        id: "mightyiam".into(),
        classes: "".into(),
        stylesheet: None,
        content: html! {
            a href="https://github.com/mightyiam" { "Shahar Dawn Or (mightyiam)" }
        },
    }
}
