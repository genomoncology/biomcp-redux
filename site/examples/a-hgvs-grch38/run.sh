biomcp variant normalize --source car --hgvs 'NM_004985.5:c.35G>A' \
  | jq '.result.results[0]
    | {outcome, mappings: [.mappings[]
        | select(.field | test("^@id$|GRCh38")) | {field, value}]}'
