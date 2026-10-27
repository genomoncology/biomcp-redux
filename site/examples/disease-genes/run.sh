biomcp get disease MONDO:0005105 genes --source monarch --section-limit 3 \
  | jq -r '.result.items[] | .association | [.subject.label, .predicate, .object.label, .primary_knowledge_source] | @tsv'
