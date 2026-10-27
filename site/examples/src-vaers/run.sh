biomcp search adverse-event MMR --source vaers --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result)'
