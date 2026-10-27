biomcp pathway genes R-HSA-5674135 | jq -r '.result.header.name, (.result.items[0:5][] | [.reference.value, .name, .gene] | @tsv)'
