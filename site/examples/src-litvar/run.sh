biomcp search article --variant rs113488022 --source litvar --limit 2 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | {pmid, title: .header.title})'
