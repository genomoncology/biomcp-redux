biomcp search cell-line A-375 --limit 20 \
  | jq -r '.result.items[] | select(.name == "A-375")
    | [.accession, .name, .category, .diseases.items[0].label] | @tsv'
