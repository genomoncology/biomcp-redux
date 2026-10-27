biomcp search trial --condition melanoma --variant "BRAF V600E" \
  --status RECRUITING --limit 3 \
  | jq -r 'def cut: if length > 56 then .[:56] + "…" else . end;
    .result.items[] | "\(.nct_id)  \((.title | cut))"'
