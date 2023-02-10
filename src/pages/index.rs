use super::base;
use crate::{
    constants::{DEFAULT_BRANCH, MOBS_PATH, REPO_URL},
    mobs::{self, get_all_participants, MobId, MobTitle},
    pages::calendar,
    style::{BUTTON_CLASSES, BUTTON_GAP},
};
use chrono::{DateTime, Utc};
use maud::{html, Markup};
use ssg::{Asset, Source, Targets};

pub async fn page() -> Asset {
    let mobs = mobs::read_all_mobs().await;
    Asset::new("/index.html".into(), async {
        let mut existing_mobs_url = REPO_URL.clone();
        existing_mobs_url
            .path_segments_mut()
            .unwrap()
            .push("tree")
            .push(DEFAULT_BRANCH)
            .push(MOBS_PATH);

        let participants = get_all_participants().await;

        Source::BytesWithAssetSafety(Box::new(move |targets| {
            let events = mobs
                .iter()
                .flat_map(|mob| mob.events(&targets, event_content_template))
                .collect();
            let calendar_html = calendar::markup(&targets, events);
            let content = html! {
                (calendar_html)
                div class=(classes!("flex" "flex-wrap" format!("gap-x-{BUTTON_GAP}"))) {
                    a
                        class=(*BUTTON_CLASSES)
                        href=(existing_mobs_url)
                        { "Add your mob" }
                }
                div class=(classes!("flex" "flex-wrap")) {
                    @for person in participants {
                        @if let Some(avatar_url) = &person.avatar_url {
                            a href=(person.social_url) class=(classes!("w-20")) {
                                img alt=(person.name) src=(avatar_url);
                            }
                        }
                    }
                }
            };
            Ok(
                base(None, content, classes!("flex" "flex-col" "gap-1"), &targets)
                    .0
                    .into_bytes(),
            )
        }))
    })
}

fn event_content_template(
    _start: DateTime<Utc>,
    _end: DateTime<Utc>,
    mob_id: MobId,
    mob_title: MobTitle,
    targets: &Targets,
) -> Markup {
    let target_path = targets.path_of(format!("/mobs/{mob_id}.html")).unwrap();
    html! {
        a class=(classes!("no-underline" "block" "h-full")) href=(target_path) {
            (mob_title)
        }
    }
}
