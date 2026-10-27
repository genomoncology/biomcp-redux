biomcp search drug artesunate --source who-pq --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {reference, product: .inn_dosage_form_strength})'
