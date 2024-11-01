use chrono::Duration;
use csscolorparser::Color;
use maud::{html, Markup, PreEscaped, Render};
use once_cell::sync::Lazy;
use rrule::RRuleSet;
use serde::{Serialize, Serializer};
use serde_json::json;

use crate::html::css_class;
use crate::mob;
use crate::relative_path::RelativePathBuf;
use crate::style::{BUTTON_CLASSES, BUTTON_GAP, TEXT_COLOR};
use crate::url::Url;

pub(crate) struct Calendar {
    events: Vec<CalendarEvent>,
    status_legend: Option<mob::status::Legend>,
    fullcalendar_path: RelativePathBuf,
    rrule_path: RelativePathBuf,
    fullcalendar_rrule_path: RelativePathBuf,
}

impl Calendar {
    pub(crate) fn new(
        events: Vec<CalendarEvent>,
        status_legend: Option<mob::status::Legend>,
        fullcalendar_path: RelativePathBuf,
        rrule_path: RelativePathBuf,
        fullcalendar_rrule_path: RelativePathBuf,
    ) -> Self {
        Self {
            events,
            status_legend,
            fullcalendar_path,
            rrule_path,
            fullcalendar_rrule_path,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CalendarEvent {
    rrule: RRuleSet,
    #[serde(serialize_with = "serialize_duration")]
    duration: Duration,
    event_content: String,
    background_color: Color,
    text_color: Color,
}

impl CalendarEvent {
    pub(crate) fn new(
        rrule: RRuleSet,
        duration: Duration,
        event_content: String,
        background_color: Color,
        text_color: Color,
    ) -> Self {
        Self {
            rrule,
            duration,
            event_content,
            background_color,
            text_color,
        }
    }
}

fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let in_whole_hours = duration.num_hours();
    let in_minutes = duration.num_minutes();
    let hour_remainder_in_minutes = in_minutes - in_whole_hours * 60;

    serializer.serialize_str(&format!(
        "{in_whole_hours:02}:{hour_remainder_in_minutes:02}"
    ))
}

const FULLCALENDAR_VERSION: &str = "6.0.2";

pub(crate) static LIBRARY_URL: Lazy<Url> = Lazy::new(|| {
    Url::parse(
        &(format!(
            "https://cdn.jsdelivr.net/npm/fullcalendar@{FULLCALENDAR_VERSION}/index.global.min.js"
        )),
    )
    .unwrap()
});

pub(crate) static RRULE_URL: Lazy<Url> = Lazy::new(|| {
    Url::parse("https://cdn.jsdelivr.net/npm/rrule@2.7.2/dist/es5/rrule.min.js").unwrap()
});

pub(crate) static FULLCALENDAR_RRULE_URL: Lazy<Url> = Lazy::new(|| {
    Url::parse(&format!("https://cdn.jsdelivr.net/npm/@fullcalendar/rrule@{FULLCALENDAR_VERSION}/index.global.min.js")).unwrap()
});

impl Render for Calendar {
    fn render(&self) -> maud::Markup {
        #[derive(Debug, PartialEq, Eq)]
        enum Direction {
            Left,
            Right,
        }

        fn arrow(direction: &Direction) -> Markup {
            let mut classes = classes!("w-[1em]", format!("fill-{TEXT_COLOR}"));

            if direction == &Direction::Right {
                classes.push("rotate-180".parse().unwrap());
            }

            html! {
                svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 32 32"
                    class=(classes) {
                        path d="M13,25.6c-0.5,0-1-0.2-1.4-0.6l-8.3-8.3c-0.4-0.4-0.4-1,0-1.4L11.6,7c0.6-0.6,1.4-0.7,2.2-0.4c0.8,0.3,1.2,1,1.2,1.8V12h12 c1.1,0,2,0.9,2,2v4c0,1.1-0.9,2-2,2H15v3.6c0,0.8-0.5,1.5-1.2,1.8C13.5,25.5,13.3,25.6,13,25.6z" {}
                                    }
            }
        }

        const CALENDAR_FN_SNIPPET: &str = include_str!("calendar/snippet.js");
        const INPUT_ATTR: &str = "data-input";
        let calendar_container_class = css_class();
        let date_range_class = css_class();
        let timezone_class = css_class();
        let button_prev_class = css_class();
        let button_next_class = css_class();
        let button_today_class = css_class();

        let calendar_fn_input = json!({
            "events": self.events,
            "selectors": {
                "calendarContainer": format!(".{calendar_container_class}"),
                "dateRange": format!(".{date_range_class}"),
                "timezone": format!(".{timezone_class}"),
                "buttonPrev": format!(".{button_prev_class}"),
                "buttonNext": format!(".{button_next_class}"),
                "buttonToday": format!(".{button_today_class}"),
            },
        });

        let input_selector = format!("[{INPUT_ATTR}]");

        let top_classes = classes!(
            "flex",
            "justify-between",
            "items-center",
            "flex-wrap",
            format!("gap-x-{BUTTON_GAP}")
        );

        let timezone_and_dates_classes = classes!(
            "flex",
            "flex-wrap",
            "gap-x-[1ch]",
            "whitespace-nowrap",
            "flex-1"
        );

        html! {
            div class=(top_classes) {
                div class=(timezone_and_dates_classes) {
                    p class=(classes!(timezone_class)) {}
                    p class=(classes!(date_range_class)) {}
                }

                div class=(classes!("flex" ,format!("gap-x-{BUTTON_GAP}"))) {
                    div class=({BUTTON_CLASSES.clone() + classes!(button_prev_class)}) {
                        (arrow(&Direction::Left))
                    }

                    div class=({BUTTON_CLASSES.clone() + classes!(button_next_class)}) {
                        (arrow(&Direction::Right))
                    }

                    button class=({BUTTON_CLASSES.clone() + classes!(button_today_class)}) { "Today" }
                }
            }

            @if let Some(status_legend) = &self.status_legend {
                (status_legend)
            }

            div class=(classes!(calendar_container_class, "[--fc-page-bg-color:transparent]")) {}
            script defer src=(self.fullcalendar_path) {}
            script defer src=(self.rrule_path) {}
            script defer src=(self.fullcalendar_rrule_path) {}
            script data-input=(calendar_fn_input.to_string()) {
                (PreEscaped(format!("
                    const input = JSON.parse(document.querySelector('{input_selector}').getAttribute('{INPUT_ATTR}'))
                    {CALENDAR_FN_SNIPPET}(input)
                ")))
            }
        }
    }
}
