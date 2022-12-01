mod index;
mod join;
mod publish;
use super::COPYRIGHT_HOLDER;
use crate::{fonts, NAME};
use chrono::{Datelike, Utc};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use ssg::{Asset, Targets};
use std::vec;

pub(crate) fn base(
    title: String,
    content: Markup,
    stylesheets: impl IntoIterator<Item = String>,
    content_classes: String,
    targets: &Targets,
) -> Markup {
    let version = Utc::now().timestamp_millis();
    let content_classes =
        content_classes + " " + &["grow", "flex", "flex-col", "justify-center"].join(" ");
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
        body."min-h-screen"."p-1".flex."flex-col"."gap-1" {
            div.flex."justify-end"."flex-wrap"."gap-x-2"."gap-y-1".uppercase."text-lg" {
                div."flex-1".flex."flex-wrap" {
                    div."flex-initial"."flex"."flex-col"."gap-x-2"."whitespace-nowrap" {
                        a."tracking-widest"."text-center" href="/" { (NAME) }
                        p."text-sm"."text-slate-700" { "A mob programming community" }
                    }
                }
                a href=(targets.relative("join.html").unwrap().to_str().unwrap()) { "Join" }
                a href=(targets.relative("publish.html").unwrap().to_str().unwrap()) { "Publish" }
                a href="https://github.com/mobusoperandi" { "GitHub" }
            }
            hr {}
            div class=(content_classes) {
                (content)
            }
            hr {}
            p {
                ({
                    let year = chrono::Utc::now().year();
                    format!("© {year} {COPYRIGHT_HOLDER}")
                })
                ", licensed "
                a href="https://tldrlegal.com/license/mit-license" { "MIT" }
                "."
            }
        }
      }
    };
    markup
}

pub(crate) async fn all() -> Vec<Asset> {
    vec![index::page().await, join::page(), publish::page()]
}
