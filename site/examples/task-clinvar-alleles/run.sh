biomcp get variant rs113488022 \
  | jq -r '.result.candidates[]
    | [(.identity.ids | join(" ")),
    .identity.representations[0].expression.value] | @tsv'
