biomcp get gene BRAF constraint | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[0].sections.constraint | {source, pli: .data.pli, loeuf: .data.loeuf})'
