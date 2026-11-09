biomcp variant assertions clinvar:13961 --source clinvar --limit 50 \
  | jq -r '(.result.assertions | group_by(.kind)
      | map("\(.[0].kind) \(length)") | join(", ")) as $kinds
    | "total: \(.result.page.total)", "this page: \($kinds)",
    ([.result.assertions[] | select(.kind == "scv")][0:4][]
      | [.statement.attribution.record, .domain,
         .statement.classification.label, .statement.attribution.evaluated]
      | @tsv)'
