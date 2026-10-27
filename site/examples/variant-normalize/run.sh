biomcp variant normalize --source car --hgvs 'NM_004333.4:c.1799T>A' \
  | jq -r '.result.results[0] | "outcome: \(.outcome)", (.mappings[0:4][] | "\(.field)\t\(.value)")'
