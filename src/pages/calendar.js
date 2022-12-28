/* global FullCalendar */
; (() => {
  const styleElm = document.createElement('style');
  document.head.append(styleElm);
  styleElm.sheet.insertRule('.fc .fc-toolbar .fc-toolbar-title { font-size: inherit }')
  styleElm.sheet.insertRule('.fc .fc-toolbar.fc-header-toolbar { margin-bottom: 0.5em; }')
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
    contentHeight: 'auto',
    eventMinHeight: 40,
    nowIndicator: true,
    eventTextColor: 'black',
    eventBackgroundColor: 'gray',
    eventBorderColor: 'gray'
  })
  calendar.render()
  document.querySelector('.fc-toolbar-title').prepend(
    `Time zone: ${Intl.DateTimeFormat().resolvedOptions().timeZone}`,
    document.createElement('br'),
  )
})()
