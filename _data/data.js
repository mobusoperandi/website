const { Temporal, toTemporalInstant } = require('@js-temporal/polyfill')
Date.prototype.toTemporalInstant = toTemporalInstant
const yaml = require('yaml')
const fs = require('fs').promises
const path = require('path')
const { RRule } = require('rrule')
module.exports = async () => {
  const filePath = path.join(__dirname, 'data.yaml')
  const fileContent = (await fs.readFile(filePath)).toString()
  const data = yaml.parse(fileContent)
  const events = JSON.stringify(data.flatMap(mob => mob.schedule.flatMap(schedule => {
    const [startHour, startMinute] = schedule.start.split(':')
    const rule = new RRule({
      ...RRule.parseText(schedule.rrule),
      tzid: schedule.timezone,
      dtstart: new Date(
        new Temporal.ZonedDateTime(0n, Temporal.TimeZone.from(schedule.timeZone))
          .with({ hour: startHour, minute: startMinute })
          .epochMilliseconds
      )
    })
    const occurrences = rule.between(
      new Date(
        Temporal.Now.instant()
          .subtract(Temporal.Duration.from({ hours: 7 * 24 }))
          .epochMilliseconds
      ),
      new Date(
        Temporal.Now.instant()
          .add(Temporal.Duration.from({ hours: 100 * 24 }))
          .epochMilliseconds
      ),
      true
    )
    const events = occurrences.map(occurrence => {
      const [durationHours, durationMinutes] = schedule.duration.split(':')
      return {
        start: occurrence,
        end: new Date(
          occurrence.toTemporalInstant()
            .add({ hours: durationHours, minutes: durationMinutes })
            .epochMilliseconds
        ),
        title: mob.name,
        url: `/mobs/${mob.id}.html`,
        backgroundColor: mob.theme.bgColor,
        textColor: mob.theme.textColor
      }
    })
    return events
  })))
  return { mobs: data, events }
}
