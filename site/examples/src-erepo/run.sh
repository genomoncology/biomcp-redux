biomcp get variant CA000434 erepo \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.candidates[0].sections.erepo.data.assertions[]
      | {gene: .statement.gene_context.label,
         condition: .statement.condition.label,
         classification: .statement.classification.label,
         panel: .statement.attribution.submitter})'
