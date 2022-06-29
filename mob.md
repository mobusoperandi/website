---
pagination:
  data: data.mobs
  size: 1
  alias: mob
permalink: "mobs/{{ mob.id }}.html"
---

{% extends "base.njk" %}
{% block content %}

# {{ mob.name }}

{% for pattern in mob.schedule %}
- {{ pattern.rrule }} at {{ pattern.start }} for {{ pattern.duration }} hours, {{ pattern.timeZone }}.
{% endfor %}

{{ mob.description }}

{% endblock %}
