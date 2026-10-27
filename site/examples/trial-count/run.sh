biomcp search trial --condition melanoma --status RECRUITING --count-only | jq -c '{status, sources: [.sources[] | {source, count}]}'
