biomcp search diagnostic --source gtr --gene KRAS --limit 3 \
  | jq -r '"tests: \(.page.total)", (.result.items[] | [.header.id, .header.name, .header.laboratory.name, .matched.relation.test_type] | @tsv)'
