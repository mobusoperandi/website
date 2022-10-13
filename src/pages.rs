mod in_the_media;
mod index;
mod join;
mod mightyiam;
mod mobs_calendar;
mod why_mob;
use crate::fonts;
use chrono::Utc;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use ssg::Asset;
use std::vec;

const NAME: &str = "Mobus Operandi";

pub(crate) fn base(
    content: Markup,
    stylesheets: impl IntoIterator<Item = String>,
    html_classes: String,
    body_classes: String,
) -> Markup {
    let version = Utc::now().timestamp_millis();
    let html_classes = ["[font-size:20px]", &html_classes].join(" ");
    let body_classes = ["p-1", &body_classes].join(" ");
    let markup = html! {
      (DOCTYPE)
      html lang="en" class=(format!("font-[{}] {html_classes}", fonts::VOLLKORN)) {
        head {
          title { (NAME) }
          meta charset="utf-8";
          meta name="viewport" content="width=device-width, initial-scale=1.0";
          link rel="stylesheet" href={ "/index.css?v=" (version) };
          @for stylesheet in stylesheets {
              link rel="stylesheet" href=(stylesheet);
          }
          style {
            // TODO extract as font utility
            @for font in fonts::ALL {(PreEscaped(format!("
              @font-face {{
                font-family: '{}';
                src: url('/{}') format('truetype');
              }}
            ", font.name, fonts::output_filename(&font))))}
          }
        }
        body class=(body_classes) {
            (content)
        }
      }
    };
    markup
}

pub(crate) async fn all() -> Vec<Asset> {
    vec![
        in_the_media::page(),
        index::page(),
        join::page(),
        mightyiam::page(),
        mobs_calendar::page().await,
        why_mob::page(),
    ]
}
