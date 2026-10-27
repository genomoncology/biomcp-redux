biomcp search article --gene KRAS --variant G12C --drug sotorasib --type review --limit 3 | jq -r '.result.items[] | [.pmid, .header.date, .header.title] | @tsv'
