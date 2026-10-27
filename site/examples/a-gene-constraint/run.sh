biomcp get gene TP53 constraint \
  | jq -c '.result.items[0].sections.constraint | {source, version: .data.version, transcript: .data.transcript, pli: .data.pli, loeuf: .data.loeuf}'
