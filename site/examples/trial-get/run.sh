biomcp get trial NCT06887088 \
  | jq 'def cut: if length > 50 then .[:50] + "…" else . end;
    .result.items[0].card
      | {nct_id, title: (.title | cut), status, phases, conditions}'
