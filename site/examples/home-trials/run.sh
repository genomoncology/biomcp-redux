biomcp search trial --condition melanoma --variant "BRAF V600E" --status RECRUITING --limit 3 \
  | jq -r '.result.items[] | "\(.nct_id)  \(.title)"'
