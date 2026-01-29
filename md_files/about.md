{% extends "layout.html" %} {% block title %}About{% endblock %} {% block
content %}

{% for title, content in keywords.json.items() %}

# {{ title }}

{{ content }}

{% endfor %}

{% endblock %}

