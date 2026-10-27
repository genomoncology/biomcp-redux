biomcp search adverse-event vemurafenib --limit 2 | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.items[] | {id, serious, first_received})'
