biomcp search drug vemurafenib --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[0] | {id, name, type})'
