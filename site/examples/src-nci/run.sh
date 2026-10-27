biomcp search trial --keyword melanoma --source nci --limit 1 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.next[0].args | join(" "))'
