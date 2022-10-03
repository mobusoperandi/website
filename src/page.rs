use crate::{fonts, mobs::Event, sections};
use chrono::Utc;
use futures::Future;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::path::PathBuf;

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

pub(crate) fn index(events: Vec<Event>) -> (PathBuf, impl Future<Output = ssg::Source>) {
    (PathBuf::from("index.html"), async {
        ssg::Source::BytesWithAssetSafety(Box::new(|assets| {
            let sections = sections(assets, events);
            let content = html! {
              @for ((row, col), section) in sections.indexed_iter() {
                @let class = format!("w-screen h-screen row-start-{} col-start-{} snap-start {}", row + 1, col + 1, section.classes);
                div id=(section.id) class=(class) {
                   (section.content)
                }
              }
            };
            let stylesheets = sections
                .into_iter()
                .filter_map(|section| section.stylesheet);
            let markup = base(
                content,
                stylesheets,
                "snap-both scroll-smooth snap-proximity".to_string(),
                "grid grid-cols-auto grid-rows-auto".to_string(),
            );
            Ok(markup.0.into_bytes())
        }))
    })
}
