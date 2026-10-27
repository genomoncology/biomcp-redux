biomcp variant assertions clinvar:13961 --source clinvar --limit 50 \
  | jq -r '"total: \(.result.page.total)", "this page: \(.result.assertions | group_by(.kind) | map("\(.[0].kind) \(length)") | join(", "))",
      ([.result.assertions[] | select(.kind == "scv")][0:4][] | [.statement.attribution.record, .domain, .statement.classification.label, .statement.attribution.evaluated] | @tsv)'
