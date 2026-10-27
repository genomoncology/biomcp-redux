biomcp protein domains P15056 --limit 2 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | {id, name, entry_type})'
