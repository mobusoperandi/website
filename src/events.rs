use crate::mobs::mobs;
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz::Etc__UTC;
use csscolorparser::Color;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    title: String,
    url: String,
    background_color: Color,
    text_color: Color,
}

pub fn events() -> Vec<Event> {
    mobs()
        .into_iter()
        .flat_map(|mob| {
            mob.schedule()
                .iter()
                .cloned()
                .map(|recurring_session| (mob.clone(), recurring_session))
                .collect::<Vec<_>>()
        })
        .flat_map(|(mob, recurring_session)| {
            let (occurrences, error) = recurring_session
                .recurrence()
                .clone()
                .all_before_with_error(
                    Utc::now().with_timezone(&Etc__UTC) + Duration::weeks(10),
                    true,
                    u16::MAX,
                );
            if let Some(error) = error {
                panic!("{:?}", error);
            }
            let duration = recurring_session.duration();
            occurrences.into_iter().map(move |occurrence| Event {
                start: occurrence.with_timezone(&Utc),
                end: (occurrence + duration).with_timezone(&Utc),
                title: mob.id().to_string(),
                url: "/".to_string() + mob.id() + ".html",
                background_color: mob.background_color().clone(),
                text_color: mob.text_color().clone(),
            })
        })
        .collect()
}
