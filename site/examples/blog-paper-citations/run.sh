biomcp article citations 12068308 --limit 5 \
  | jq -r 'def cut: if length > 50 then .[:50] + "…" else . end;
    "citing papers: \(.page.total)",
    (.result.items[] | [.pmid, .header.date, (.header.title | cut)] | @tsv)'
