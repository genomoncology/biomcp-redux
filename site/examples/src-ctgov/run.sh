biomcp search trial --condition melanoma --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {nct_id, status, phases})'
