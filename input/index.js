/* global FullCalendar */
const events = window.__events
document.addEventListener('DOMContentLoaded', function init () {
  const calendarEl = document.getElementById('calendar')
  const calendar = new FullCalendar.Calendar(calendarEl, {
    initialView: 'timeGridWeek',
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
})
