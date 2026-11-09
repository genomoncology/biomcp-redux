biomcp search drug daraxonrasib --limit 1 \
  | jq '{status, sources: [.sources[].source], drug: .result.items[0]
    | {id, name, type}}'
