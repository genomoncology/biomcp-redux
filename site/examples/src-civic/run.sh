biomcp get variant CA123643 civic \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.candidates[0].sections.civic.data.profiles[0:2][]
      | {label, id: .record.id})'
