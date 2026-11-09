biomcp protein domains P15056 \
  | jq '{page: (.page | {returned, total}),
    next: [.next[].args | join(" ")]}'
