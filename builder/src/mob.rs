pub(super) mod file;
pub(super) mod id;
pub(super) mod link;
pub(super) mod participant;
pub(super) mod recurring_session;
pub(crate) mod status;
pub(super) mod subtitle;
pub(super) mod title;

use std::collections::BTreeSet;
use std::io;

use anyhow::{anyhow, Context, Result};
use chrono::DateTime;
use csscolorparser::Color;
use futures::FutureExt;
use getset::Getters;
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

use ssg::sources::bytes_with_file_spec_safety::{TargetNotFoundError, Targets};
use ssg::FileSpec;

use crate::components::{self, CalendarEvent};
use crate::constants::MOBS_PATH;
use crate::markdown::Markdown;

pub(crate) use self::file::MobFile;
pub(crate) use self::file::YamlRecurringSession;
use self::id::Id;
pub(crate) use self::link::{Link, LinkElement};
pub(crate) use self::participant::{Participant, Person};
use self::recurring_session::RecurringSession;
pub(crate) use self::status::Status;
use self::subtitle::Subtitle;
use self::title::Title;

#[derive(Debug, Clone, Getters)]
pub struct Mob {
    #[getset(get = "pub(crate)")]
    id: Id,
    #[getset(get = "pub(crate)")]
    title: Title,
    #[getset(get = "pub(crate)")]
    subtitle: Option<Subtitle>,
    #[getset(get = "pub(crate)")]
    participants: Vec<Participant>,
    schedule: Vec<RecurringSession>,
    #[getset(get = "pub(crate)")]
    freeform_copy_markdown: Markdown,
    background_color: Color,
    text_color: Color,
    links: Vec<Link>,
    #[getset(get = "pub(crate)")]
    status: Status,
}

impl TryFrom<(String, MobFile)> for Mob {
    type Error = anyhow::Error;
    fn try_from((id, yaml): (String, MobFile)) -> Result<Self, Self::Error> {
        Ok(Mob {
            id: Id::new(id),
            title: yaml.title().clone(),
            subtitle: yaml.subtitle().cloned(),
            participants: yaml.participants().clone(),
            schedule: yaml
                .schedule()
                .cloned()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            freeform_copy_markdown: yaml.freeform_copy().clone(),
            background_color: yaml.background_color().clone(),
            text_color: yaml.text_color().clone(),
            links: yaml.links().cloned().unwrap_or_default(),
            status: yaml.status().clone(),
        })
    }
}

fn read_mob(dir_entry: Result<std::fs::DirEntry, io::Error>) -> anyhow::Result<Mob> {
    let data_file_path = dir_entry?.path();
    let id = data_file_path
        .file_stem()
        .context("no filename extension")?
        .to_str()
        .context("invalid utf8")?
        .into();

    let data = std::fs::read_to_string(data_file_path.clone())?;

    let yaml_mob: MobFile =
        serde_yaml::from_str(&data).map_err(|e| anyhow!("{:?} {:?}", data_file_path, e))?;

    (id, yaml_mob).try_into()
}

type EventContentTemplate = fn(
    DateTime<rrule::Tz>,
    DateTime<rrule::Tz>,
    &Mob,
    &Targets,
) -> Result<Markup, TargetNotFoundError>;

impl Mob {
    pub(crate) fn events(
        &self,
        targets: &Targets,
        event_content_template: EventContentTemplate,
    ) -> Result<Vec<CalendarEvent>, TargetNotFoundError> {
        let events = self
            .schedule
            .iter()
            .map(|recurring_session| {
                let mob = self.clone();
                let start = *recurring_session.recurrence().get_dt_start();
                let end = start + recurring_session.duration();

                let event_content = event_content_template(start, end, &mob, targets)?;

                let background_color = mob.background_color.clone();
                let text_color = mob.text_color;

                let event_content = html! {
                    div class=(classes!("h-full", "break-words")) {
                        (event_content)
                    }
                }
                .0;

                Ok(CalendarEvent::new(
                    recurring_session.recurrence().clone(),
                    recurring_session.duration(),
                    event_content,
                    background_color,
                    text_color,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(events)
    }

    pub(super) fn page(self) -> FileSpec {
        FileSpec::new(
            format!("/mobs/{}.html", self.id),
            move |targets: Targets| {
                let mob = self.clone();

                async move {
                    let links = mob
                        .links
                        .iter()
                        .cloned()
                        .map(|link| (link, &targets).try_into())
                        .collect::<Result<Vec<LinkElement>, TargetNotFoundError>>()?;

                    let events =
                        mob.events(&targets, components::mob_page::event_content_template)?;
                    let base = components::PageBase::new(targets.clone())?;

                    let page = components::mob_page::MobPage::new(
                        mob,
                        links,
                        events,
                        base,
                        targets.path_of("/fullcalendar.js")?,
                        targets.path_of("/rrule.js")?,
                        targets.path_of("/fullcalendar_rrule.js")?,
                    );

                    Ok::<_, TargetNotFoundError>(page.render().0.into_bytes())
                }
                .boxed()
            },
        )
    }
}

pub(crate) static MOBS: Lazy<Vec<Mob>> = Lazy::new(|| {
    std::fs::read_dir(MOBS_PATH.clone())
        .unwrap()
        .map(read_mob)
        .collect::<Result<Vec<Mob>>>()
        .unwrap()
});

pub(crate) fn get_all_participants() -> BTreeSet<Person> {
    MOBS.iter()
        .flat_map(|mob| mob.participants.clone())
        .filter_map(|participant| match participant {
            Participant::Hidden => None,
            Participant::Public(person) => Some(person),
        })
        .collect()
}
