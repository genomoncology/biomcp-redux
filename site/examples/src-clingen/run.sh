biomcp get gene BRAF clingen \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[0].sections.clingen.data.validity[0:2][]
      | {disease, classification, expert_panel})'
