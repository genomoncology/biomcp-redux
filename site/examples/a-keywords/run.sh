biomcp get article 33123537 indexing \
  | jq '.result.items[0].sections.indexing
    | {source, keywords: .data.keywords, mesh: .data.mesh}'
