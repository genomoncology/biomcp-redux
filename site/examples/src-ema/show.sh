biomcp search drug Zelboraf --source ema --limit 1 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {id, name})'
