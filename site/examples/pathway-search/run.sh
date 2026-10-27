biomcp search pathway MAPK --limit 3 | jq -r '.result.items[] | [.id.source, .id.id, .name] | @tsv'
