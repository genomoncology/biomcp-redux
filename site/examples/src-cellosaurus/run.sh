biomcp get cell-line CVCL_0132 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.header | {accession, name, category})'
