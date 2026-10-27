biomcp search article 'BRAF V600E' --source europepmc --limit 2 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | {pmid, title: .header.title})'
