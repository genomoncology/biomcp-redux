biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --find AXIN2 \
  | jq -r '"\(.page.returned) of \(.page.total) rows", (.result.content.data.rows[0:3][] | [.sheet, .row, .cells[0], .cells[3], .cells[4]] | @tsv)'
