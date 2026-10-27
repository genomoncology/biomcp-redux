biomcp get phenotype HP:0001250 genes --source monarch --section-limit 3 \
  | jq -r '"total: \(.page.total)",
    (.result.items[] | .association
    | [.subject.label, .predicate, .object.label] | @tsv)'
