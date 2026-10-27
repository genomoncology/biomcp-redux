biomcp search drug vemurafenib --source drugsfda --limit 1 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[] | {id: .identity.id, brand: .products[0].brand_name})'
