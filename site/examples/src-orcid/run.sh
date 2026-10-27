biomcp get author orcid:0000-0002-1825-0097 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[0].card.header | {id, name})'
