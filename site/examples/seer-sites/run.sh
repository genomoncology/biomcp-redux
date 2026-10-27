biomcp search disease --source seer --limit 3 | jq -r '.result.items[] | [.code, .label] | @tsv'
