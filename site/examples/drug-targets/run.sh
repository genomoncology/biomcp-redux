biomcp get drug CHEMBL1229517 targets --source chembl \
  | jq '.result.targets.data.records[]
    | {target: .native_target.id, relation, mechanism}'
