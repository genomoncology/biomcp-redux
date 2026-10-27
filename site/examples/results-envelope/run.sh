biomcp search trial --condition melanoma --limit 2 \
  | jq '{schema, status, query, sources, next}
    | .sources[].terms |= if length > 50 then .[:50] + "…" else . end'
