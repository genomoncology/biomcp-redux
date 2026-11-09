biomcp get gene BRAF clingen \
  | jq -r '.result.items[0].sections.clingen.data.validity[]
    | [.disease, .classification, .mode_of_inheritance] | @tsv'
