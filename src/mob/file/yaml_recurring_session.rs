use chrono::{Duration, NaiveDate};
use chrono_tz::Tz;
use schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Schema, Clone)]
/// Specification for a recurring session
pub(crate) struct YamlRecurringSession {
    /// Frequency of the recurrence in [RRULE](https://icalendar.org/iCalendar-RFC-5545/3-8-5-3-recurrence-rule.html) format
    ///
    /// Example:
    ///
    /// ```yaml
    /// FREQ=WEEKLY;BYDAY=MO,TU,WE,TH
    /// ```
    frequency: RecurrenceFrequency,
    /// The schedule's timezone
    ///
    /// Example:
    ///
    /// ```yaml
    /// Africa/Dakar
    /// ```
    timezone: Tz,
    /// Date of the first session of this schedule
    ///
    /// Example:
    ///
    /// ```yaml
    /// 2023-02-27
    /// ```
    start_date: NaiveDate,
    /// Session start time
    ///
    /// Example:
    ///
    /// ```yaml
    /// 04:00
    /// ```
    start_time: Time,
    /// Session duration in minutes
    ///
    /// Example:
    ///
    /// ```yaml
    /// 180
    /// ```
    duration: Minutes,
}

impl YamlRecurringSession {
    pub(crate) fn frequency(&self) -> &RecurrenceFrequency {
        &self.frequency
    }

    pub(crate) fn timezone(&self) -> Tz {
        self.timezone
    }

    pub(crate) fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    pub(crate) fn start_time(&self) -> &Time {
        &self.start_time
    }

    pub(crate) fn duration(&self) -> Minutes {
        self.duration
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
pub(crate) struct Time(String);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct Minutes(u16);

impl From<Minutes> for Duration {
    fn from(minutes: Minutes) -> Self {
        Self::minutes(minutes.0.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
pub(crate) struct RecurrenceFrequency(String);
