use maud::html;

use crate::environment::DEVELOPMENT;

use super::{home, Section};

pub fn section() -> Section {
    Section::new(
        "in_the_media".into(),
        "".into(),
        None,
        html! {
          (home("row-start-1 col-start-1".into()))
          h2 class="row-start-1 col-start-2 col-end-12" { "In the media" }
          @if !DEVELOPMENT {
              iframe
                class="row-start-2 col-span-full"
                width="560"
                height="315"
                src="https://www.youtube-nocookie.com/embed/nxNDo-7Fyfk"
                title="YouTube video player"
                frameborder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                allowfullscreen {}
          }
        },
    )
}
