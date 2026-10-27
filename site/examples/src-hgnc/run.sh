biomcp search gene BRAF --limit 2 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | {hgnc_id, symbol, name})'
