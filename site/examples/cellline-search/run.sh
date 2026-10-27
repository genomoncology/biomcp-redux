biomcp search cell-line A375 --limit 3 | jq -r '.result.items[] | [.accession, .name, .category] | @tsv'
