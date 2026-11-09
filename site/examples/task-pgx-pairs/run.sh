biomcp search pgx --gene CYP2C19 --source cpic --limit 50 \
  | jq -r '([.next[]? | select(.kind == "page")] | length > 0) as $more
    | "pairs: \(.result.items | length), more pages: \($more)",
    (.result.items[] | select(.cpiclevel == "A")
      | [.drugname, .cpiclevel, .clinpgxlevel] | @tsv)'
