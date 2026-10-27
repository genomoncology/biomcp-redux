biomcp article citations 33123537 --limit 3 | jq -r '.result.items[] | [.pmid, .header.date, .header.title] | @tsv'
