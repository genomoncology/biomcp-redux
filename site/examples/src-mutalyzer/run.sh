biomcp variant normalize --source mutalyzer --hgvs 'NM_004333.4:c.1799T>A' \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.results[]
      | {source, outcome, normalized,
         protein: [.mappings[]
           | select(.field == "protein.description") | .value][0]})'
