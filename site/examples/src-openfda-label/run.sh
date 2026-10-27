biomcp search drug Zelboraf --source openfda-label --limit 1 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | {id, name, version})'
