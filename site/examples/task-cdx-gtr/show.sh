biomcp search diagnostic --source gtr --gene KRAS --limit 3 \
  | jq -r 'def cut: if length > 24 then .[:24] + "…" else . end;
    "tests: \(.page.total)",
    (.result.items[]
    | [.header.id, (.header.name | cut), (.header.laboratory.name | cut),
    .matched.relation.test_type] | @tsv)'
