biomcp get variant 'chr7:g.140453136A>T' --build GRCh37 clinvar \
  | jq -r '.result.candidates[0].sections.clinvar.data.assertions[]
    | select(.kind == "vcv")
    | [.domain, .statement.classification.label,
    .statement.attribution.record] | @tsv'
