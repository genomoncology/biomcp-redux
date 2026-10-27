biomcp article references 12068308 --limit 5 \
  | jq -r '"references: \(.page.total)", "sources: \([.sources[].source] | join(", "))", (.result.items[] | [.pmid, .header.date, .header.title[0:70]] | @tsv)'
