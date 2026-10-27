biomcp search trial --condition "non-small cell lung cancer" \
  --variant "KRAS G12C" --status RECRUITING --limit 3 \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    .result.items[] | {nct_id, title: (.title | cut)}'
