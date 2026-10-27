biomcp protein domains P15056 | jq -c '{page: (.page | {returned, total}), next: [.next[].args | join(" ")]}'
