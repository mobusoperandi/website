use super::Section;
use maud::html;
pub fn section() -> Section {
    Section {
        id: "main".into(),
        classes: [
            "text-center",
            "uppercase",
            "tracking-widest",
            "text-[6vw]",
            "flex",
            "flex-col",
            "justify-around",
        ]
        .join(" "),
        stylesheet: None,
        content: html! {
            h2 {
                span { "Study" }
                " "
                span { "software development" }
                " "
                span { "online" }
                " in "
                span { "mob programming" }
                " format."
            }
            a href="#mobs_calendar" { "See existing mobs" }
        },
    }
}
