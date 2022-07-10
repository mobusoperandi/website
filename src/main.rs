mod events;
mod markdown_to_html;
mod mobs;
mod sections;
use crate::sections::sections;
use chrono::offset::Utc;
use maud::{html, Markup, DOCTYPE};
use mobs::mobs;
use std::{fs, path::PathBuf};

const NAME: &str = "Mobus Operandi";

#[derive(Clone)]
struct Page {
    path: PathBuf,
    markup: Markup,
}

fn base(
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

fn index() -> Page {
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
    Page {
        path: "index.html".into(),
        markup,
    }
}

fn main() {
    let index_page = index();
    let mob_pages = mob_pages();
    [[index_page].as_slice(), mob_pages.as_slice()]
        .concat()
        .into_iter()
        .for_each(|Page { path, markup }| {
            let output_dir_path: PathBuf = std::env::var("OUTPUT_DIR").unwrap().parse().unwrap();
            let output_file_path: PathBuf = [output_dir_path, path].into_iter().collect();
            fs::write(output_file_path, markup.into_string()).unwrap();
        })
}

fn mob_pages() -> Vec<Page> {
    mobs()
        .into_iter()
        .map(|mob| {
            let description = mob.description();
            let content = html! {
                h1 { (mob.id()) }
                (*description)
            };
            Page {
                path: [mob.id(), ".html"].concat().parse().unwrap(),
                markup: base(content, [].into_iter(), "".to_string(), "".to_string()),
            }
        })
        .collect()
}
