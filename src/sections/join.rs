use maud::html;

use super::Section;

pub fn section() -> Section {
    Section::new(
        "join".into(),
        "".into(),
        None,
        html! {
            p {
                "Join by " a href="https://calendly.com/mightyiam" { "scheduling a chat with Dawn" }
            }
        },
    )
}
