use std::collections::BTreeSet;
use std::io;

use anyhow::anyhow;
use chrono::{DateTime, Duration, Utc};
use chrono::{NaiveDate, TimeZone};
use chrono_tz::Tz;
use csscolorparser::Color;
use futures::StreamExt;
use maud::{html, Markup, PreEscaped, Render};
use rrule::{RRule, RRuleSet, Unvalidated};
use serde::Deserialize;
use serde::Serialize;
use ssg::Targets;
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;

use crate::components::CalendarEvent;
use crate::constants::MOBS_PATH;
use crate::markdown::Markdown;
use crate::url::Url;

#[derive(Debug, Clone)]
pub struct Mob {
    pub(crate) id: MobId,
    pub(crate) title: MobTitle,
    pub(crate) subtitle: Option<MobSubtitle>,
    pub(crate) participants: Vec<MobParticipant>,
    pub(crate) schedule: Vec<RecurringSession>,
    pub(crate) freeform_copy_markdown: Markdown,
    pub(crate) background_color: Color,
    pub(crate) text_color: Color,
    pub(crate) links: Vec<Link>,
    pub(crate) status: Status,
}

#[derive(Debug, Clone, derive_more::Display)]
pub(crate) struct MobId(String);
#[derive(Debug, Clone, derive_more::Display, Serialize, Deserialize)]
pub(crate) struct MobTitle(String);

impl MobTitle {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Render for MobTitle {
    fn render(&self) -> Markup {
        PreEscaped(self.0.clone())
    }
}

#[derive(Debug, Clone, derive_more::Display, Serialize, Deserialize)]
pub(crate) struct MobSubtitle(String);

impl Render for MobSubtitle {
    fn render(&self) -> Markup {
        html! { p { (self.0) } }
    }
}

impl TryFrom<(String, MobFile)> for Mob {
    type Error = anyhow::Error;
    fn try_from((id, yaml): (String, MobFile)) -> Result<Self, Self::Error> {
        Ok(Mob {
            id: MobId(id),
            title: yaml.title,
            subtitle: yaml.subtitle,
            participants: yaml.participants,
            schedule: yaml
                .schedule
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            freeform_copy_markdown: yaml.freeform_copy,
            background_color: yaml.background_color,
            text_color: yaml.text_color,
            links: yaml.links.unwrap_or_default(),
            status: yaml.status,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Status {
    Short(Markdown),
    Open(Markdown),
    Full(Option<Markdown>),
    Public(Markdown),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Link {
    YouTube(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum MobParticipant {
    Hidden,
    Public(Person),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Person {
    pub(crate) name: PersonName,
    pub(crate) social_url: Url,
    pub(crate) avatar_url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct PersonName(String);

impl Render for PersonName {
    fn render(&self) -> Markup {
        self.0.render()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecurringSession {
    pub(crate) recurrence: RRuleSet,
    pub(crate) duration: Duration,
}

#[derive(Deserialize)]
struct MobFile {
    title: MobTitle,
    subtitle: Option<MobSubtitle>,
    participants: Vec<MobParticipant>,
    schedule: Vec<YamlRecurringSession>,
    background_color: Color,
    text_color: Color,
    links: Option<Vec<Link>>,
    freeform_copy: Markdown,
    status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
struct RecurrenceFrequency(String);

#[derive(Deserialize)]
struct YamlRecurringSession {
    frequency: RecurrenceFrequency,
    timezone: Tz,
    start_date: NaiveDate,
    start_time: Time,
    duration: Minutes,
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
struct Time(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Minutes(u16);

impl From<Minutes> for Duration {
    fn from(minutes: Minutes) -> Self {
        Self::minutes(minutes.0.into())
    }
}

impl TryFrom<YamlRecurringSession> for RecurringSession {
    type Error = anyhow::Error;
    fn try_from(yaml_recurring_session: YamlRecurringSession) -> Result<Self, Self::Error> {
        let YamlRecurringSession {
            frequency: recurrence,
            timezone,
            start_date,
            start_time,
            duration,
        } = yaml_recurring_session;

        let recurrence = format!("RRULE:{recurrence}");
        let rrule: RRule<Unvalidated> = recurrence.parse()?;
        let timezone: rrule::Tz = timezone.into();

        let start_date_time = timezone
            .datetime_from_str(&format!("{start_date}{start_time}"), "%F%R")
            .unwrap();

        let recurrence = rrule.build(start_date_time).unwrap();
        let duration = duration.into();

        Ok(RecurringSession {
            recurrence,
            duration,
        })
    }
}

pub(crate) async fn read_mob(dir_entry: Result<fs::DirEntry, io::Error>) -> Mob {
    let data_file_path = dir_entry.unwrap().path();
    let id = data_file_path.file_stem().unwrap().to_str().unwrap().into();
    let data = fs::read_to_string(data_file_path.clone()).await.unwrap();

    let yaml_mob: MobFile = serde_yaml::from_str(&data)
        .map_err(|e| anyhow!("{:?} {:?}", data_file_path, e))
        .unwrap();

    (id, yaml_mob).try_into().unwrap()
}

impl Mob {
    pub(crate) fn events(
        &self,
        targets: &Targets,
        event_content_template: fn(
            DateTime<Utc>,
            DateTime<Utc>,
            MobId,
            MobTitle,
            &Targets,
        ) -> Markup,
    ) -> Vec<CalendarEvent> {
        self.schedule
            .iter()
            .flat_map(|recurring_session| {
                let duration = recurring_session.duration;
                let mob = self.clone();

                recurring_session
                    .recurrence
                    .into_iter()
                    .map(move |occurrence| {
                        let start = occurrence.with_timezone(&Utc);
                        let end = (start + duration).with_timezone(&Utc);

                        let event_content = event_content_template(
                            start,
                            end,
                            mob.id.clone(),
                            mob.title.clone(),
                            targets,
                        );

                        let background_color = mob.background_color.clone();
                        let text_color = mob.text_color.clone();

                        let event_content = html! {
                            div class=(classes!("h-full" "break-words")) {
                                (event_content)
                            }
                        }
                        .0;

                        CalendarEvent {
                            start,
                            end,
                            event_content,
                            background_color,
                            text_color,
                        }
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

pub(crate) async fn get_all_participants() -> BTreeSet<Person> {
    read_all_mobs()
        .await
        .into_iter()
        .flat_map(|mob| mob.participants)
        .filter_map(|participant| match participant {
            MobParticipant::Hidden => None,
            MobParticipant::Public(person) => Some(person),
        })
        .collect()
}
