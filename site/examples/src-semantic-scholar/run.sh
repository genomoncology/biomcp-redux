biomcp article recommendations 33123537 --limit 2 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | {pmid, title: .header.title})'
