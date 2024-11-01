use chrono::{Duration, NaiveDate};
use chrono_tz::Tz;
use getset::{CopyGetters, Getters};
use schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Schema, Clone, Getters, CopyGetters)]
/// Specification for a recurring session
pub(crate) struct YamlRecurringSession {
    /// Frequency of the recurrence in [RRULE](https://icalendar.org/iCalendar-RFC-5545/3-8-5-3-recurrence-rule.html) format
    ///
    /// Example:
    ///
    /// ```yaml
    /// FREQ=WEEKLY;BYDAY=MO,TU,WE,TH
    /// ```
    #[getset(get = "pub(crate)")]
    frequency: RecurrenceFrequency,
    /// The schedule's timezone
    ///
    /// Example:
    ///
    /// ```yaml
    /// Africa/Dakar
    /// ```
    #[getset(get_copy = "pub(crate)")]
    timezone: Tz,
    /// Date of the first session of this schedule
    ///
    /// Example:
    ///
    /// ```yaml
    /// 2023-02-27
    /// ```
    #[getset(get_copy = "pub(crate)")]
    start_date: NaiveDate,
    /// Session start time
    ///
    /// Example:
    ///
    /// ```yaml
    /// 04:00
    /// ```
    #[getset(get = "pub(crate)")]
    start_time: Time,
    /// Session duration in minutes
    ///
    /// Example:
    ///
    /// ```yaml
    /// 180
    /// ```
    #[getset(get_copy = "pub(crate)")]
    duration: Minutes,
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
