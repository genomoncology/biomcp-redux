biomcp get gene BRAF \
  | jq '{status, gene: .result.items[0].answer.gene
    | {symbol, hgnc_id, name, location},
    sources: [.sources[] | {source, state, reason}]}'
