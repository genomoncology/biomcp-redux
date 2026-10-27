biomcp get disease MONDO:0005105 --source mydisease | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.result.concept)'
