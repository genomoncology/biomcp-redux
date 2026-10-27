biomcp search pgx --gene CYP2C19 --source cpic --limit 50 \
  | jq -r '"pairs: \(.result.items | length), more pages: \([.next[]? | select(.kind == "page")] | length > 0)", (.result.items[] | select(.cpiclevel == "A") | [.drugname, .cpiclevel, .clinpgxlevel] | @tsv)'
