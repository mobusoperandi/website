pub(crate) mod yaml_recurring_session;

use csscolorparser::Color;
use schema::Schema;
use serde::Deserialize;

use crate::markdown::Markdown;

pub(crate) use self::yaml_recurring_session::YamlRecurringSession;

use super::{subtitle::Subtitle, title::Title, Link, Participant, Status};

#[derive(Deserialize, Schema)]
/// The contents of a mob file
pub(crate) struct MobFile {
    /// The mob's title
    ///
    /// Example:
    ///
    /// ```yaml
    /// Agile Bandits
    /// ```
    title: Title,
    /// An optional mob's subtitle
    ///
    /// Example:
    ///
    /// ```yaml
    /// Hackin' and cruisin'
    /// ```
    subtitle: Option<Subtitle>,
    /// Regular participants of the mob
    participants: Vec<Participant>,
    /// The mob's regular schedule
    schedule: Vec<YamlRecurringSession>,
    /// Color of the background of calendar event blocks
    ///
    /// Example:
    ///
    /// ```yaml
    /// aliceblue
    /// ```
    background_color: Color,
    /// Color of text inside calendar event blocks
    ///
    /// Example:
    ///
    /// ```yaml
    /// orangered
    /// ```
    text_color: Color,
    /// Links associated with the mob
    ///
    /// ```yaml
    /// - !YouTube @agilebandits
    /// ```
    links: Option<Vec<Link>>,
    /// A description of the mob, the purpose of it, its past attainments, etc.
    ///
    /// ```yaml
    /// ## What we do
    ///
    /// We study the BrainShock programming language.
    /// ```
    freeform_copy: Markdown,
    /// The mob's current status
    ///
    /// Example:
    ///
    /// ```yaml
    /// !Public |
    ///   ## Just show up!
    ///
    ///   [Room link](https://meet.jit.si/MedievalWebsPortrayLoud)
    /// ```
    status: Status,
}

impl MobFile {
    pub(crate) fn title(&self) -> &Title {
        &self.title
    }

    pub(crate) fn subtitle(&self) -> Option<&Subtitle> {
        self.subtitle.as_ref()
    }

    pub(crate) fn participants(&self) -> &Vec<Participant> {
        &self.participants
    }

    pub(crate) fn schedule(&self) -> impl Iterator<Item = &YamlRecurringSession> {
        self.schedule.iter()
    }

    pub(crate) fn freeform_copy(&self) -> &Markdown {
        &self.freeform_copy
    }

    pub(crate) fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub(crate) fn text_color(&self) -> &Color {
        &self.text_color
    }

    pub(crate) fn links(&self) -> Option<&Vec<Link>> {
        self.links.as_ref()
    }

    pub(crate) fn status(&self) -> &Status {
        &self.status
    }
}
