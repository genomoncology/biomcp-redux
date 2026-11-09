biomcp variant normalize --source variantvalidator \
  --hgvs 'NM_004333.4:c.1799T>A' --build GRCh38 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.results[] | {source, outcome})'
