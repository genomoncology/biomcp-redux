biomcp get drug CHEMBL1229517 indications --source opentargets \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.indications.data.records[0:2][]
      | {indication, stage: .stage.value})'
