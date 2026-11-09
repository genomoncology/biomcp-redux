biomcp get disease MONDO:0005105 genes --source monarch --section-limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | .association
      | {gene: .subject.label, disease: .object.label})'
