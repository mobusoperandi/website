/* global FullCalendar */
function initCalendar({ events, displayEventTime, selectors }) {
  window.addEventListener("DOMContentLoaded", () => {
    const styleElm = document.createElement("style");

    document.head.append(styleElm);

    styleElm.sheet.insertRule(
      ".fc .fc-toolbar .fc-toolbar-title { font-size: inherit }",
    );
    styleElm.sheet.insertRule(
      ".fc .fc-toolbar.fc-header-toolbar { margin-bottom: 0.5em; }",
    );
    styleElm.sheet.insertRule(".fc .fc-timegrid-slot { height: 2.5em; }");
    styleElm.sheet.insertRule(
      ".fc .fc-col-header-cell-cushion { text-decoration: none; }",
    );

    const calendarContainerElm = document.querySelector(
      selectors.calendarContainer,
    );
    const calendar = new FullCalendar.Calendar(calendarContainerElm, {
      initialView: "timeGridWeek",
      slotDuration: "01:00:00",
      expandRows: true,
      dayHeaderFormat: { weekday: "short" },
      views: {
        timeGridWeek: {
          allDaySlot: false,
        },
      },
      events,
      eventContent: ({ event }) => ({ html: event.extendedProps.eventContent }),
      height: "auto",
      contentHeight: "auto",
      eventMinHeight: 40,
      nowIndicator: true,
      displayEventTime,
      eventBorderColor: "transparent",
      headerToolbar: false,
      stickyHeaderDates: false,
    });

    const dateRangeElm = document.querySelector(selectors.dateRange);
    const timezoneElm = document.querySelector(selectors.timezone);

    calendar.on("datesSet", (dateInfo) => {
      const start = new Intl.DateTimeFormat().format(dateInfo.start);
      const end = new Intl.DateTimeFormat().format(dateInfo.end);
      dateRangeElm.textContent = `${start} – ${end}`;
      timezoneElm.textContent = `Time zone: ${Intl.DateTimeFormat().resolvedOptions().timeZone}`;
    });

    const buttonPrevElm = document.querySelector(selectors.buttonPrev);

    buttonPrevElm.addEventListener("click", () => {
      calendar.prev();
    });

    const buttonNextElm = document.querySelector(selectors.buttonNext);

    buttonNextElm.addEventListener("click", () => {
      calendar.next();
    });

    const buttonTodayElm = document.querySelector(selectors.buttonToday);

    buttonTodayElm.addEventListener("click", () => {
      calendar.today();
    });

    calendar.render();
  });
}

// prettier-ignore
initCalendar
