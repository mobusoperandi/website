use chrono::{Duration, NaiveDateTime};
use getset::{CopyGetters, Getters};
use rrule::{RRule, RRuleSet, Unvalidated};

use super::file::yaml_recurring_session::YamlRecurringSession;

#[derive(Debug, Clone, PartialEq, Eq, Getters, CopyGetters)]
pub(crate) struct RecurringSession {
    #[getset(get = "pub(crate)")]
    recurrence: RRuleSet,
    #[getset(get_copy = "pub(crate)")]
    duration: Duration,
}

impl TryFrom<YamlRecurringSession> for RecurringSession {
    type Error = anyhow::Error;
    fn try_from(yaml_recurring_session: YamlRecurringSession) -> Result<Self, Self::Error> {
        let recurrence = yaml_recurring_session.frequency();
        let timezone = yaml_recurring_session.timezone();
        let start_date = yaml_recurring_session.start_date();
        let start_time = yaml_recurring_session.start_time();
        let duration = yaml_recurring_session.duration();

        let recurrence = format!("RRULE:{recurrence}");
        let rrule: RRule<Unvalidated> = recurrence.parse()?;
        let timezone: rrule::Tz = timezone.into();

        let start_date_time =
            NaiveDateTime::parse_from_str(&format!("{start_date}{start_time}"), "%F%R")?
                .and_local_timezone(timezone)
                .unwrap()
                // workaround for https://github.com/fullcalendar/fullcalendar/issues/6815
                // timezones with non-zero offset result in occurrences with wrong datetimes
                .with_timezone(&rrule::Tz::UTC);

        let recurrence = rrule
            // workaround for https://github.com/fullcalendar/fullcalendar/issues/6834
            // no ocurrences generated for recurring events with TZID and without UNTIL
            // so we add an arbitrary UNTIL
            .until((start_date_time + Duration::days(365 * 99)).with_timezone(&rrule::Tz::UTC))
            .build(start_date_time)?;

        let duration = duration.into();

        Ok(RecurringSession {
            recurrence,
            duration,
        })
    }
}
