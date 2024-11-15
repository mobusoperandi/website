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
use camino::Utf8Path;
use chrono::DateTime;
use csscolorparser::Color;
use getset::Getters;
use maud::{html, Markup, Render};

use ssg_child::sources::BytesSource;
use ssg_child::sources::ExpectedFiles;
use ssg_child::FileSpec;

use crate::components::{self, CalendarEvent};
use crate::expected_files::ExpectedFilesExt;
use crate::markdown::Markdown;
use crate::relative_path::RelativePathBuf;

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

fn read_mob(dir_entry: Result<std::fs::DirEntry, io::Error>) -> Mob {
    let data_file_path = dir_entry.unwrap().path();
    let id = data_file_path
        .file_stem()
        .context("no filename extension")
        .unwrap()
        .to_str()
        .context("invalid utf8")
        .unwrap()
        .into();

    let data = std::fs::read_to_string(data_file_path.clone()).unwrap();

    let yaml_mob: MobFile = serde_yaml::from_str(&data)
        .map_err(|e| anyhow!("{:?} {:?}", data_file_path, e))
        .unwrap();

    (id, yaml_mob).try_into().unwrap()
}

type EventContentTemplate =
    fn(DateTime<rrule::Tz>, DateTime<rrule::Tz>, &Mob, &mut ExpectedFiles) -> Markup;

impl Mob {
    pub(crate) fn events(
        &self,
        expected_files: &mut ExpectedFiles,
        event_content_template: EventContentTemplate,
    ) -> Vec<CalendarEvent> {
        let events = self
            .schedule
            .iter()
            .map(|recurring_session| {
                let mob = self.clone();
                let start = *recurring_session.recurrence().get_dt_start();
                let end = start + recurring_session.duration();

                let event_content = event_content_template(start, end, &mob, expected_files);

                let background_color = mob.background_color.clone();
                let text_color = mob.text_color;

                let event_content = html! {
                    div class=(classes!("h-full", "break-words")) {
                        (event_content)
                    }
                }
                .0;

                CalendarEvent::new(
                    recurring_session.recurrence().clone(),
                    recurring_session.duration(),
                    event_content,
                    background_color,
                    text_color,
                )
            })
            .collect::<Vec<_>>();

        events
    }

    pub(super) fn page(self) -> FileSpec {
        let path = RelativePathBuf::from(format!("/mobs/{}.html", self.id));
        let mut expected_files = ExpectedFiles::default();

        let links = self
            .links
            .iter()
            .cloned()
            .map(|link| (link, &mut expected_files).into())
            .collect::<Vec<LinkElement>>();

        let events = self.events(
            &mut expected_files,
            components::mob_page::event_content_template,
        );

        let markup = if let status::Status::Renamed(renamed_id) = self.status() {
            let base = components::PageBase::new(&mut expected_files, path.clone());

            let page = components::redirect_page::RedirectPage::new(
                base,
                format!("/mobs/{renamed_id}.html"),
            );

            page.render()
        } else {
            let base = components::PageBase::new(&mut expected_files, path.clone());

            let page = components::mob_page::MobPage::new(
                self,
                links,
                events,
                base,
                expected_files.insert_("/fullcalendar.js"),
                expected_files.insert_("/rrule.js"),
                expected_files.insert_("/fullcalendar_rrule.js"),
            );

            page.render()
        };

        let bytes = markup.render().0.into_bytes();

        FileSpec::new(path, BytesSource::new(bytes, Some(expected_files)))
    }
}

pub(crate) fn get_all(mobs_path: &Utf8Path) -> impl IntoIterator<Item = Mob> {
    std::fs::read_dir(mobs_path).unwrap().map(read_mob)
}

pub(crate) fn get_all_participants(mobs: &[Mob]) -> BTreeSet<Person> {
    mobs.iter()
        .flat_map(|mob| mob.participants.clone())
        .filter_map(|participant| match participant {
            Participant::Hidden => None,
            Participant::Public(person) => Some(person),
        })
        .collect()
}
