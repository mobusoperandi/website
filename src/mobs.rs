use crate::MOBS_PATH;
use chrono::TimeZone;
use chrono::{DateTime, Duration, Utc};
use csscolorparser::Color;
use futures::StreamExt;
use rrule::{RRule, RRuleSet, Unvalidated};
use serde::Deserialize;
use serde::Serialize;
use ssg::Targets;
use std::path::PathBuf;
use std::{io, path::Path};
use tokio::fs::{self, read_to_string};
use tokio_stream::wrappers::ReadDirStream;
use url::Url;

#[derive(Debug, Clone)]
pub struct Mob {
    #[allow(unused)]
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) subtitle: Option<String>,
    pub(crate) participants: Vec<MobParticipant>,
    pub(crate) schedule: Vec<RecurringSession>,
    pub(crate) freeform_copy_markdown: String,
    pub(crate) background_color: Color,
    pub(crate) text_color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum MobParticipant {
    Hidden,
    Public(Person),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Person {
    pub(crate) name: String,
    pub(crate) social_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecurringSession {
    pub(crate) recurrence: RRuleSet,
    pub(crate) duration: Duration,
}

#[derive(Deserialize)]
struct YAMLMob {
    title: String,
    subtitle: Option<String>,
    participants: Vec<MobParticipant>,
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

async fn read_mob_data_file(
    path: &Path,
) -> (
    String,
    Option<String>,
    Vec<MobParticipant>,
    Vec<RecurringSession>,
    Color,
    Color,
) {
    let data = fs::read_to_string(path).await.unwrap();
    let yaml_mob: YAMLMob = serde_yaml::from_str(&data).unwrap();
    let schedule = yaml_mob.schedule.into_iter().map(Into::into).collect();
    (
        yaml_mob.title,
        yaml_mob.subtitle,
        yaml_mob.participants,
        schedule,
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
    let data_dir_path = dir_entry.unwrap().path();
    let id = data_dir_path.file_stem().unwrap().to_str().unwrap().into();
    let (title, subtitle, participants, schedule, background_color, text_color) =
        read_mob_data_file(
            &[data_dir_path.clone(), "data.yaml".into()]
                .iter()
                .collect::<PathBuf>(),
        )
        .await;
    let freeform_copy_markdown = read_to_string(
        &[data_dir_path, "freeform_copy.md".into()]
            .iter()
            .collect::<PathBuf>(),
    )
    .await
    .unwrap();
    Mob {
        title,
        subtitle,
        id,
        participants,
        schedule,
        background_color,
        text_color,
        freeform_copy_markdown,
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

impl Mob {
    pub(crate) fn events(&self, targets: &Targets, titles: bool) -> Vec<Event> {
        self.schedule
            .iter()
            .flat_map(|recurring_session| {
                let duration = recurring_session.duration;
                let mob = self.clone();
                recurring_session
                    .recurrence
                    .into_iter()
                    .map(move |occurrence| Event {
                        start: occurrence.with_timezone(&Utc),
                        end: (occurrence + duration).with_timezone(&Utc),
                        title: if titles {
                            mob.title.clone()
                        } else {
                            "".to_owned()
                        },
                        url: targets.path_of(format!("mobs/{}.html", mob.id)).unwrap(),
                        background_color: mob.background_color.clone(),
                        text_color: mob.text_color.clone(),
                    })
            })
            .take_while(|event| {
                event.start <= Utc::now().with_timezone(&rrule::Tz::Etc__UTC) + Duration::weeks(10)
            })
            .collect()
    }
}

pub(crate) async fn read_all_mobs() -> Vec<Mob> {
    ReadDirStream::new(fs::read_dir(MOBS_PATH).await.unwrap())
        .then(read_mob)
        .collect::<Vec<_>>()
        .await
}
