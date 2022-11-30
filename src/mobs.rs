use crate::mobs;
use chrono::TimeZone;
use chrono::{DateTime, Duration, Utc};
use csscolorparser::Color;
use futures::StreamExt;
use rrule::{RRule, RRuleSet, Unvalidated};
use serde::Deserialize;
use serde::Serialize;
use std::{io, path::Path};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;
use url::Url;

#[derive(Debug, Clone)]
pub struct Mob {
    pub(crate) id: String,
    pub(crate) schedule: Vec<RecurringSession>,
    pub(crate) url: Url,
    pub(crate) background_color: Color,
    pub(crate) text_color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecurringSession {
    pub(crate) recurrence: RRuleSet,
    pub(crate) duration: Duration,
}

#[derive(Deserialize)]
struct YAMLMob {
    schedule: Vec<YAMLRecurringSession>,
    url: Url,
    background_color: Color,
    text_color: Color,
}
#[derive(Deserialize)]
struct YAMLRecurringSession {
    recurrence: String,
    timezone: String,
    start_date: String,
    start_time: String,
    duration: u16,
}

async fn read_mob_data_file(path: &Path) -> (Vec<RecurringSession>, Url, Color, Color) {
    let data = fs::read_to_string(path).await.unwrap();
    let yaml_mob: YAMLMob = serde_yaml::from_str(&data).unwrap();
    let schedule = yaml_mob.schedule.into_iter().map(Into::into).collect();
    (
        schedule,
        yaml_mob.url,
        yaml_mob.background_color,
        yaml_mob.text_color,
    )
}

impl From<YAMLRecurringSession> for RecurringSession {
    fn from(yaml_recurring_session: YAMLRecurringSession) -> Self {
        let YAMLRecurringSession {
            recurrence,
            timezone,
            start_date,
            start_time,
            duration,
        } = yaml_recurring_session;
        let recurrence = format!("RRULE:{recurrence}");
        let rrule: RRule<Unvalidated> = recurrence.parse().unwrap();
        let timezone: chrono_tz::Tz = timezone.parse().unwrap();
        let timezone: rrule::Tz = timezone.into();
        let start_date_time = timezone
            .datetime_from_str(&(start_date + &start_time), "%F%R")
            .unwrap();
        let recurrence = rrule.build(start_date_time).unwrap();
        let duration = Duration::minutes(duration.into());
        RecurringSession {
            recurrence,
            duration,
        }
    }
}

pub(crate) async fn read_mob(dir_entry: Result<fs::DirEntry, io::Error>) -> Mob {
    let data_file_path = dir_entry.unwrap().path();
    let id = data_file_path.file_stem().unwrap().to_str().unwrap().into();
    let (schedule, url, background_color, text_color) = read_mob_data_file(&data_file_path).await;
    Mob {
        id,
        schedule,
        url,
        background_color,
        text_color,
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    title: String,
    url: Url,
    background_color: Color,
    text_color: Color,
}

pub(crate) fn events(mut events: Vec<Event>, mob: mobs::Mob) -> Vec<Event> {
    mob.schedule
        .iter()
        .flat_map(|recurring_session| {
            let duration = recurring_session.duration;
            let mob = mob.clone();
            recurring_session
                .recurrence
                .into_iter()
                .map(move |occurrence| Event {
                    start: occurrence.with_timezone(&Utc),
                    end: (occurrence + duration).with_timezone(&Utc),
                    title: mob.id.clone(),
                    url: mob.url.clone(),
                    background_color: mob.background_color.clone(),
                    text_color: mob.text_color.clone(),
                })
        })
        .take_while(|event| {
            event.start <= Utc::now().with_timezone(&rrule::Tz::Etc__UTC) + Duration::weeks(10)
        })
        .for_each(|event| events.push(event));
    events
}
pub(crate) async fn read_all_mobs() -> Vec<Mob> {
    ReadDirStream::new(fs::read_dir("mobs").await.unwrap())
        .then(read_mob)
        .collect::<Vec<_>>()
        .await
}
