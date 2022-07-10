use chrono::{Duration, TimeZone};
use chrono_tz::Tz;
use csscolorparser::Color;
use maud::{html, Markup, PreEscaped};
use rrule::{RRule, RRuleSet, Unvalidated};
use serde::Deserialize;
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

use crate::markdown_to_html::markdown_to_html;

#[derive(Clone)]
pub struct Mob {
    id: String,
    schedule: Vec<RecurringSession>,
    description: Markup,
    background_color: Color,
    text_color: Color,
}

impl Mob {
    pub fn schedule(&self) -> &[RecurringSession] {
        &self.schedule
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn text_color(&self) -> &Color {
        &self.text_color
    }

    pub fn description(&self) -> &Markup {
        &self.description
    }
}

#[derive(Clone)]
pub struct RecurringSession {
    recurrence: RRuleSet,
    duration: Duration,
}

impl RecurringSession {
    pub fn recurrence(&self) -> &RRuleSet {
        &self.recurrence
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }
}

const MOBS_PATH: &str = "mobs";

pub fn mobs() -> Vec<Mob> {
    read_dir(MOBS_PATH)
        .unwrap()
        .into_iter()
        .map(|entry| {
            let entry = entry.unwrap();
            let mob_id = entry.file_name();
            let data_path: PathBuf = [entry.path().to_str().unwrap(), "data.yaml"]
                .into_iter()
                .collect();
            let data = read_to_string(data_path).unwrap();
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
            let yaml_mob: YAMLMob = serde_yaml::from_str(&data).unwrap();
            let schedule = yaml_mob
                .schedule
                .into_iter()
                .map(|yaml_recurring_session| {
                    let YAMLRecurringSession {
                        recurrence,
                        timezone,
                        start_date,
                        start_time,
                        duration,
                    } = yaml_recurring_session;
                    let recurrence = format!("RRULE:{recurrence}");
                    let rrule: RRule<Unvalidated> = recurrence.parse().unwrap();
                    let timezone: Tz = timezone.parse().unwrap();
                    let start_date_time = timezone
                        .datetime_from_str(&(start_date + &start_time), "%F%R")
                        .unwrap();
                    let rrule = rrule.build(start_date_time).unwrap();
                    let duration = Duration::minutes(duration.into());
                    RecurringSession {
                        recurrence: rrule,
                        duration,
                    }
                })
                .collect();
            let description_path: PathBuf = [entry.path(), "description.md".parse().unwrap()]
                .into_iter()
                .collect();
            let description = read_to_string(description_path).unwrap();
            let description = markdown_to_html(&description);
            let description = html!((PreEscaped(description)));
            Mob {
                id: mob_id.into_string().unwrap(),
                schedule,
                description,
                background_color: yaml_mob.background_color,
                text_color: yaml_mob.text_color,
            }
        })
        .collect()
}
