biomcp article references 12068308 --limit 5 \
  | jq -r 'def cut: if length > 50 then .[:50] + "…" else . end;
    "references: \(.page.total)",
    "sources: \([.sources[].source] | join(", "))",
    (.result.items[] | [.pmid, .header.date, (.header.title | cut)] | @tsv)'
