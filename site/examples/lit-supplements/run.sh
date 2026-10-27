biomcp get article 28944238 supplements \
  | jq -r '.result.items[0].sections.supplements.data.files[]
    | [.name, .kind] | @tsv'
