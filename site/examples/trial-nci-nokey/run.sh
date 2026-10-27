biomcp search trial --keyword melanoma --source nci --limit 1 \
  | jq '{status, sources: [.sources[] | {source, state, reason}], next}'
