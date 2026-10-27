biomcp search pathway MAPK --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {id: .id.id, name})'
