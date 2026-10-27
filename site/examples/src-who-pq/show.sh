biomcp search drug artesunate --source who-pq --limit 2 \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    {status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {reference, product: (.inn_dosage_form_strength | cut)})'
