biomcp search trial --condition melanoma --has-results --count-only | jq -c '{status, sources: [.sources[] | {source, count}]}'
