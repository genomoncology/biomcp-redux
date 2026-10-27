biomcp search trial --intervention daraxonrasib --limit 20 \
  | jq -r '"total: \(.page.total)", (.result.items[] | [.nct_id, .status, (.phases | join(",")), .title[0:60]] | @tsv)'
