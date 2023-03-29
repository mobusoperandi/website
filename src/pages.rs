pub(crate) mod add;
mod index;

use std::vec;

use futures::FutureExt;
use maud::Render;
use ssg::sources::bytes_with_file_spec_safety::{TargetNotFoundError, Targets};
use ssg::FileSpec;

use crate::components::mob_page::{event_content_template, MobPage};
use crate::{
    mobs::{self, Mob},
    url::Url,
};

fn mob_page(mob: Mob) -> FileSpec {
    FileSpec::new(format!("/mobs/{}.html", mob.id), move |targets: Targets| {
        let mob = mob.clone();

        async {
            let links = mob
                .links
                .iter()
                .map(|link| match link {
                    mobs::Link::YouTube(path) => {
                        let mut url = Url::parse("https://www.youtube.com").unwrap();
                        url.set_path(path);
                        Ok((url, targets.path_of("/youtube_logo.svg")?, "YouTube"))
                    }
                })
                .collect::<Result<Vec<_>, TargetNotFoundError>>()?;

            let events = mob.events(&targets, event_content_template)?;

            let page = MobPage {
                mob,
                targets,
                links,
                events,
            };

            Ok::<_, TargetNotFoundError>(page.render().0.into_bytes())
        }
        .boxed()
    })
}

pub(crate) async fn all() -> Vec<FileSpec> {
    let mobs = mobs::read_all_mobs().await;
    let mut mob_pages = mobs.iter().cloned().map(mob_page).collect::<Vec<_>>();
    let mut pages = vec![index::page().await, add::page()];

    pages.append(&mut mob_pages);

    pages
}
