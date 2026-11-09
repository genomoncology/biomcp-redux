biomcp article supplement 28944238 MGG3-5-553-s002.xlsx --variant P245S \
  | jq -r '"\(.page.returned) of \(.page.total) rows",
    (.result.content.data.rows[]
    | [.sheet, .row, .cells[0], .cells[3], .cells[4]] | @tsv)'
