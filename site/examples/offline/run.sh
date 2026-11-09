biomcp get gene BRAF > /dev/null
biomcp --offline get gene BRAF \
  | jq '{status, sources: [.sources[] | {source, state, reason}]}'
biomcp --offline search trial --condition glioblastoma --limit 1 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]}'
