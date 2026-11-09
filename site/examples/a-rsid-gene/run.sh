biomcp get variant rs113488022 \
  | jq '.result.candidates[] | {ids: .identity.ids,
    genes: [.identity.genes[]?.label],
    hgvs: [.identity.representations[]? | .expression.value]}'
