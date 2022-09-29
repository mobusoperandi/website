use crate::{environment::OUTPUT_DIR, fonts, mobs::Event, sections, ssg};
use chrono::Utc;
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
          style type="text/css" {
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

pub(crate) fn index(events: Vec<Event>) -> ssg::Input {
    let sections = sections(events);
    let content = html! {
      @for ((row, col), section) in sections.indexed_iter() {
        @let class = format!("w-screen h-screen row-start-{} col-start-{} snap-start {}", row + 1, col + 1, section.classes);
        section id=(section.id) class=(class) {
           (section.content)
        }
      }
    };
    let stylesheets = sections
        .into_iter()
        .filter_map(|section| section.stylesheet.map(|stylesheel| stylesheel.to_owned()));
    let markup = base(
        content,
        stylesheets,
        "snap-both scroll-smooth snap-proximity".to_string(),
        "grid grid-cols-auto grid-rows-auto".to_string(),
    );
    ssg::Input {
        target_path: PathBuf::from(OUTPUT_DIR).join("index.html"),
        source: ssg::Source::Bytes(markup.0.into_bytes()),
    }
}
