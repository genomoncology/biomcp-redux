biomcp search variant --gene BRAF --protein V600E \
  | jq '.result.items[] | .identity
    | {ids, genes: [.genes[].label],
    hgvs: [.representations[] | "\(.build) \(.expression.value)"]}'
