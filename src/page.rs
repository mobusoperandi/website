use chrono::Utc;
use maud::{html, Markup, DOCTYPE};

use crate::{mobs, out, sections};

const NAME: &str = "Mobus Operandi";

pub(crate) fn base(
    content: Markup,
    stylesheets: impl Iterator<Item = String>,
    html_classes: String,
    body_classes: String,
) -> Markup {
    let version = Utc::now().timestamp_millis();
    let markup = html! {
      (DOCTYPE)
      html lang="en" class=(html_classes) {
        head {
          title { (NAME) }
          meta charset="utf-8";
          meta name="viewport" content="width=device-width, initial-scale=1.0";
          link rel="stylesheet" href={ "/index.css?v=" (version) };
          @for stylesheet in stylesheets {
              link rel="stylesheet" href=(stylesheet);
          }
        }
        body class=(body_classes) {
            (content)
        }
      }
    };
    markup
}

pub(crate) fn index() -> out::File {
    let sections = sections();
    let content = html! {
      @for ((row, col), section) in sections.indexed_iter() {
        @let id = section.id();
        @let classes = section.classes();
        @let class = format!("w-screen h-screen row-start-{} col-start-{} snap-start {classes}", row + 1, col + 1);
        @let content = section.content();
        section id=(id) class=(class) {
           (content)
        }
      }
    };
    let stylesheets = sections
        .into_iter()
        .filter_map(|section| section.stylesheet().map(|stylesheel| stylesheel.to_owned()));
    let markup = base(
        content,
        stylesheets,
        "snap-both scroll-smooth snap-proximity".to_string(),
        "grid grid-cols-auto grid-rows-auto".to_string(),
    );
    out::File {
        target_path: "index.html".into(),
        source: out::Source::Markup(markup),
    }
}

pub(crate) fn mob_pages() -> Vec<out::File> {
    mobs()
        .into_iter()
        .map(|mob| {
            let description = mob.description();
            let content = html! {
                h1 { (mob.id()) }
                (*description)
            };
            out::File {
                target_path: [mob.id(), ".html"].concat().parse().unwrap(),
                source: out::Source::Markup(base(
                    content,
                    [].into_iter(),
                    "".to_string(),
                    "".to_string(),
                )),
            }
        })
        .collect()
}
