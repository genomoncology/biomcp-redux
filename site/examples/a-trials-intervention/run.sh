biomcp search trial --intervention sotorasib --status RECRUITING --limit 3 | jq -r '.result.items[] | [.nct_id, .title] | @tsv'
