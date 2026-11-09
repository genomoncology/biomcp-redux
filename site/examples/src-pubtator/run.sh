biomcp get article 33123537 annotations \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[0].sections.annotations.data.items[0:3][]
      | {kind, name, count})'
