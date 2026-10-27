biomcp sync ema > /dev/null
biomcp search drug Zelboraf --source ema --limit 1 | jq -r '.result.items[] | [.id, .name] | @tsv'
biomcp get drug EMEA/H/C/002409 regulatory --source ema \
  | jq -r '.result.regulatory.data.records[] | "status: \(.status)", (.dates[] | select(.role == "marketing_authorisation_date") | "\(.role): \(.date)")'
