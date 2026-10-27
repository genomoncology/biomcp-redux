biomcp get variant rs7903146 | jq -r '.result.candidates[] | [(.identity.ids | join(" ")), .identity.representations[0].build, .identity.representations[0].expression.value] | @tsv'
biomcp variant normalize --source car --hgvs 'NC_000010.10:g.114758349C>T' \
  | jq -r '.result.results[0] | "outcome: \(.outcome)", (.mappings[0:2][] | "\(.field)\t\(.value)")'
