biomcp get gene BRAF pathways \
  | jq -r '.result.items[0].sections.pathways | "source: \(.source)",
    (.data.items[0:5][] | "\(.id)\t\(.name)")'
