biomcp search diagnostic --source gtr --gene BRAF --limit 2 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | .header | {id, name, lab: .laboratory.name})'
