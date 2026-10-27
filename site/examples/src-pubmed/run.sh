biomcp search article --gene BRAF --variant V600E --limit 2 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | {pmid, title: .header.title})'
