biomcp get variant clinvar:13961 clinvar \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.candidates[0].sections.clinvar.data.assertions[0:2][]
      | {kind, domain, classification: .statement.classification.label})'
