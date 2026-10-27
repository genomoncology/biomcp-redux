biomcp get gene BRAF | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[0].answer.gene.summary | {source, dated, text: .text[0:100]})'
