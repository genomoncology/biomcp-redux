biomcp search trial --condition "non-small cell lung cancer" --variant "KRAS G12C" --status RECRUITING --limit 3 \
  | jq -r '.result.items[] | [.nct_id, .title] | @tsv'
