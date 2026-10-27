biomcp get phenotype HP:0001250 --source hpo \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.term | {id: .returned_id, label})'
