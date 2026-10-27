biomcp search adverse-event vemurafenib --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {id, serious, first_received})'
