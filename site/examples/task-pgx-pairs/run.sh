biomcp search pgx --gene CYP2C19 --source cpic --limit 30 \
  | jq -r '"pairs: \(.result.items | length)", (.result.items[] | select(.cpiclevel == "A") | [.drugname, .cpiclevel, .clinpgxlevel] | @tsv)'
