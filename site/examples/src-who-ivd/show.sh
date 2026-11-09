biomcp search diagnostic --source who-ivd --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {id, product_name, marker})'
