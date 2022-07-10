/* global FullCalendar */
(() => {
  document.addEventListener('DOMContentLoaded', function init () {
    const calendarEl = document.querySelector('#mobs_calendar > div')
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
      height: '100%',
      nowIndicator: true,
      eventTextColor: 'black',
      eventBackgroundColor: 'gray',
      eventBorderColor: 'gray'
    })
    calendar.render()
  })
})()
