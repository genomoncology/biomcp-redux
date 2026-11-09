biomcp search article "BRAF V600E" --source europepmc --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, count}],
    pmids: [.result.items[].key.id]}'
