mod calendar;
mod index;
use crate::fonts;
use chrono::Utc;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use ssg::{Asset, Targets};
use std::vec;

const NAME: &str = "Mobus Operandi";

pub(crate) fn base(
    title: String,
    content: Markup,
    stylesheets: impl IntoIterator<Item = String>,
    content_classes: String,
    targets: &Targets,
) -> Markup {
    let version = Utc::now().timestamp_millis();
    let markup = html! {
      (DOCTYPE)
      html lang="en" class=(format!("font-[{}] [font-size:16px]", fonts::VOLLKORN)) {
        head {
          title { (format!("{title}; {NAME}")) }
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
        body."min-h-screen"."p-1".flex."flex-col" {
            div."mb-2".grid."grid-flow-col"."grid-cols-[auto_1fr]"."gap-2".uppercase."text-lg" {
                a href="/" { (NAME) }
                a."col-start-3" href=(targets.relative("calendar.html").unwrap().to_str().unwrap()) {
                    "Calendar"
                }
                a."col-start-4" href="https://github.com/mobusoperandi" { "GitHub" }
            }
            div class=(content_classes) {
                (content)
            }
        }
      }
    };
    markup
}

pub(crate) async fn all() -> Vec<Asset> {
    vec![index::page(), calendar::page().await]
}
