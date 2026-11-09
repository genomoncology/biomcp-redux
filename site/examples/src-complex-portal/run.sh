biomcp protein complexes P15056 --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {id, participant_total, predicted})'
