biomcp get variant 'chr12:g.25398285C>A' --build GRCh37 clinvar \
  | jq -r '.result.candidates[0].sections.clinvar.data.assertions[] | select(.kind == "vcv") | [.domain, .statement.classification.label] | @tsv'
