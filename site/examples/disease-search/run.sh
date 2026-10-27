biomcp search disease melanoma --limit 3 | jq -r '.result.items[] | [.concept.id, .concept.label] | @tsv'
