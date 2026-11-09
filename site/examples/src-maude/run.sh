biomcp search adverse-event pacemaker --source maude --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {id, event_type, first_received})'
