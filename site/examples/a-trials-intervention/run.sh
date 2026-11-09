biomcp search trial --intervention sotorasib --status RECRUITING --limit 3 \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    .result.items[] | {nct_id, title: (.title | cut)}'
