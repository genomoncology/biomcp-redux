biomcp search article "BRAF V600E" --source europepmc --limit 2 | jq -c '{status, sources: [.sources[] | {source, state}], pmids: [.result.items[].key.id]}'
