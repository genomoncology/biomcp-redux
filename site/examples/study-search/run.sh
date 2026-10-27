biomcp search study melanoma --limit 3 | jq -c '{status, sources: [.sources[] | {source, state, reason}], next: [.next[]?.args | join(" ")]}'
