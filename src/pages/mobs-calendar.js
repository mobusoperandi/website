/* global FullCalendar */
; (() => {
  const calendarEl = document.querySelector('div:empty')
  const calendar = new FullCalendar.Calendar(calendarEl, {
    initialView: 'timeGridWeek',
    slotDuration: '01:00:00',
    expandRows: true,
    dayHeaderFormat: { weekday: 'short' },
    views: {
      timeGridWeek: {
        allDaySlot: false
      }
    },
    events,
    height: 'auto',
    nowIndicator: true,
    eventTextColor: 'black',
    eventBackgroundColor: 'gray',
    eventBorderColor: 'gray'
  })
  calendar.render()
})()
