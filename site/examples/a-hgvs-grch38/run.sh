biomcp variant normalize --source car --hgvs 'NM_004985.5:c.35G>A' \
  | jq -r '.result.results[0] | "outcome: \(.outcome)", (.mappings[] | select(.field | test("^@id$|GRCh38")) | "\(.field)\t\(.value)")'
