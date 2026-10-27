biomcp sync wikipathways | jq -c '{status, sources: [.sources[] | {source, state}]}'
biomcp search pathway MAPK --source wikipathways --limit 3 | jq -r '.result.items[] | [.id.id, .name] | @tsv'
