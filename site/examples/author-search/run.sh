biomcp search author "Louis Williams" --limit 3 | jq -r '.result.items[] | [.id, .name] | @tsv'
