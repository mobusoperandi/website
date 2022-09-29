use maud::html;

use super::Section;

pub fn section() -> Section {
    Section {
        id: "join".into(),
        classes: "".into(),
        stylesheet: None,
        content: html! {
            p {
                "Join by " a href="https://calendly.com/mightyiam" { "scheduling a chat with Dawn" }
            }
        },
    }
}
