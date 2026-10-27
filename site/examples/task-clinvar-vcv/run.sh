biomcp get variant clinvar:13961 clinvar \
  | jq -r '.result.candidates[0].sections.clinvar.data.assertions[] | select(.kind == "vcv") | [.domain, .statement.classification.label, .statement.strength.label, .statement.attribution.evaluated] | @tsv'
