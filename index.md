{% extends "base.njk" %}

{% block head %}
<link href='https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.css' rel='stylesheet' />
<script src='https://cdn.jsdelivr.net/npm/fullcalendar@5.11.0/main.min.js'></script>
<script>
  window.__events = {{ data.events | safe }}
</script>
<script defer src='/index.js'></script>
{% endblock %}

{% block title %}Home{% endblock %}

{% block content %}

A small community for the regular study of [the Rust programming language][rust] in [remote mob programming] format — and it's free.

https://youtu.be/nxNDo-7Fyfk

# How do I join?

Please [schedule a chat with Dawn][schedule].

# Why join?

Because you'll learn and level-up on numerous skills:

- Communication
- Collaboration
- Rust
- Knowledge sharing
- Various development tools and workflows

You'll make friends.

You'll have fun.

You may build something you'll be proud of.

# Existing mobs on a calendar

<div id='calendar'></div>

# Who is the founder?

[Shahar Dawn Or (mightyiam)][mightyiam]

[schedule]: https://calendly.com/mightyiam
[rust]: https://www.rust-lang.org/
[remote mob programming]: https://remotemobprogramming.org/
[mightyiam]: https://github.com/mightyiam
[calendar]: https://calendar.google.com/calendar/u/0/embed?src=e7v8tv7rcfmp1mde6l8dhk9uts@group.calendar.google.com&mode=week&showTabs=0
[timezones]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones

{% endblock %}
