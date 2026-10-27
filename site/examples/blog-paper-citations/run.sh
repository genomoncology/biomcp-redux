biomcp article citations 12068308 --limit 5 \
  | jq -r '"citing papers: \(.page.total)", (.result.items[] | [.pmid, .header.date, .header.title[0:70]] | @tsv)'
