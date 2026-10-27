biomcp search phenotype seizure --limit 3 | jq -r '.result.items[] | [.id, .label, .match_type] | @tsv'
