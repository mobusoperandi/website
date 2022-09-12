use chrono::Utc;
use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::{
    fonts,
    mobs::{mobs, Mob},
    out, sections,
};

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
            @for font in fonts::ALL {(PreEscaped(format!("
              @font-face {{
                font-family: '{}';
                src: url('/{}') format('truetype');
              }}
            ", font.name, font.output_filename())))}
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
    mobs().into_iter().map(mob_page).collect()
}

fn mob_page(mob: Mob) -> out::File {
    out::File {
        target_path: [mob.id(), ".html"].concat().parse().unwrap(),
        source: out::Source::Markup(base(
            html! {
                h1 { (mob.id()) }
                (*mob.description())
            },
            [].into_iter(),
            "".to_string(),
            "".to_string(),
        )),
    }
}
