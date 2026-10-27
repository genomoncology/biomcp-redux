biomcp search drug vemurafenib --source drugsfda --limit 2 \
  | jq -c '.result.items[]
    | {identity: .identity.id,
    products: [.products[]? | .brand_name // .name] | unique}'
