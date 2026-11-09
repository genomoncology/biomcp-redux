biomcp search disease --source seer --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {code, label})'
