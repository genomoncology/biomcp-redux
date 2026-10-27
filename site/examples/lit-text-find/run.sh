biomcp article text PMC7573235 --find E17K | jq -c '{page: (.page | {returned, total}), first_hits: [.result.content.data.hits[0:3][] | {part, heading_path}]}'
