biomcp search article "KRAS G12C sotorasib" --since 2024 --sort date --limit 3 | jq -r '.result.items[] | [.pmid, .header.date, .header.title] | @tsv'
