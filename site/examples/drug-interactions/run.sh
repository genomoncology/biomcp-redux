biomcp sync ddinter > /dev/null
biomcp get drug DDInter1925 interactions --source ddinter --section-limit 3 \
  | jq -r '.result.interactions.data | "total: \(.total)", (.records[] | [.partner.name, .level] | @tsv)'
