biomcp search disease melanoma --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {id: .concept.id, label: .concept.label, ontology})'
