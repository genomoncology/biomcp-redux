biomcp search trial --condition melanoma --status RECRUITING --lat=40.7128 --lon=-74.0060 --distance=50mi --limit 3 \
  | jq -r '.result.items[] | [.nct_id, .title] | @tsv'
