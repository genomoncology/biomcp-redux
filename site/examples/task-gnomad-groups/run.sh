biomcp variant population CA118786 --detail --limit 50 \
  | jq -r '"rows: \(.page.total)", (.result.groups[] | select(.data_type == "genomes" and .sex == null and (.group_id | contains(":") | not)) | [.group_id, .ac, .an] | @tsv)'
