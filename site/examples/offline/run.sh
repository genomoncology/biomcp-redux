biomcp get gene BRAF > /dev/null
biomcp --offline get gene BRAF | jq -c '{status, sources: [.sources[] | {source, state, reason}]}'
biomcp --offline search trial --condition glioblastoma --limit 1 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}'
