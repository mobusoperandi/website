use crate::markdown;
use crate::mobs;
use crate::page;
use chrono::TimeZone;
use chrono::{DateTime, Duration, Utc};
use csscolorparser::Color;
use futures::join;
use futures::Future;
use maud::html;
use maud::{Markup, PreEscaped};
use rrule::{RRule, RRuleSet, Unvalidated};
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::{io, path::Path};
use tokio::fs;

#[derive(Debug, Clone)]
pub struct Mob {
    pub(crate) id: String,
    pub(crate) schedule: Vec<RecurringSession>,
    pub(crate) description: Markup,
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

async fn read_mob_data_file(path: &Path) -> (Vec<RecurringSession>, Color, Color) {
    let data_path = path.join("data.yaml");
    let data = fs::read_to_string(data_path).await.unwrap();
    let yaml_mob: YAMLMob = serde_yaml::from_str(&data).unwrap();
    let schedule = yaml_mob.schedule.into_iter().map(Into::into).collect();
    (schedule, yaml_mob.background_color, yaml_mob.text_color)
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

async fn read_mob_description_file(path: &Path) -> Markup {
    let description_path = path.join("description.md");
    let description = fs::read_to_string(description_path).await.unwrap();
    let description = markdown::to_html(&description);
    PreEscaped(description)
}

pub(crate) async fn read_mob(dir_entry: Result<fs::DirEntry, io::Error>) -> Mob {
    let dir_path = dir_entry.unwrap().path();
    let id = dir_path.file_name().unwrap().to_str().unwrap().into();
    let ((schedule, background_color, text_color), description) = join!(
        read_mob_data_file(&dir_path),
        read_mob_description_file(&dir_path),
    );
    Mob {
        id,
        schedule,
        description,
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
    url: String,
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
                    url: format!("/{}.html", mob.id),
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

pub(crate) fn page(mob: &Mob) -> (PathBuf, impl Future<Output = ssg::Source>) {
    let mob_id = mob.id.clone();
    let mob_description = mob.description.clone();
    (PathBuf::from(mob_id.clone() + ".html"), async move {
        ssg::Source::Bytes(
            page::base(
                html! {
                    h1 { (mob_id) }
                    (mob_description)
                },
                [].into_iter(),
                "".to_string(),
                "".to_string(),
            )
            .0
            .into_bytes(),
        )
    })
}
