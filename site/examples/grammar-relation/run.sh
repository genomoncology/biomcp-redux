biomcp protein domains P15056 \
  | jq -r 'def cut: if length > 50 then .[:50] + "…" else . end;
    .result.items[] | [.id, .entry_type, (.name | cut)] | @tsv'
