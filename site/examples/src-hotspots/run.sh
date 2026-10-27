biomcp get variant clinvar:13961 hotspots \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.candidates[0].sections.hotspots.data.records[0]
      | {gene, residue, tumor_count, membership})'
