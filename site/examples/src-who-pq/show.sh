biomcp search drug artesunate --source who-pq --limit 2 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | {reference, inn_dosage_form_strength})'
