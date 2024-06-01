pub(crate) mod add_page;
mod calendar;
pub(crate) mod home_page;
pub(crate) mod mob_page;
pub(crate) mod page_base;
pub(crate) mod redirect_page;
pub(crate) mod schema;

pub(crate) use calendar::Calendar;
pub(crate) use calendar::CalendarEvent;
pub(crate) use calendar::FULLCALENDAR_RRULE_URL as CALENDAR_FULLCALENDAR_RRULE_URL;
pub(crate) use calendar::LIBRARY_URL as CALENDAR_LIBRARY_URL;
pub(crate) use calendar::RRULE_URL as CALENDAR_RRULE_URL;
pub(crate) use page_base::PageBase;
