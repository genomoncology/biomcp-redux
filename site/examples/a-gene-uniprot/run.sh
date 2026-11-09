biomcp get gene BRAF protein \
  | jq '.result.items[0].sections.protein
    | {source, proteins: [.data[] | {accession, name, length}]}'
