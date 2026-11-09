biomcp search trial --condition melanoma --status RECRUITING \
  --lat=40.7128 --lon=-74.0060 --distance=50mi --limit 3 \
  | jq 'def cut: if length > 56 then .[:56] + "…" else . end;
    .result.items[] | {nct_id, title: (.title | cut)}'
