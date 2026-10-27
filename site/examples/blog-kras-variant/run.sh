biomcp search variant --gene KRAS --protein G12C \
  | jq '{ids: .result.items[0].identity.ids,
    next: (.next[0].args | join(" "))}'
