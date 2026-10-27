biomcp search trial --intervention daraxonrasib --limit 20 \
  | jq -r 'def cut: if length > 24 then .[:24] + "…" else . end;
    "total: \(.page.total)",
    (.result.items[]
    | [.nct_id, .status, (.phases | join(",")), (.title[0:60] | cut)] | @tsv)'
