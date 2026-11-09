biomcp search variant --gene BRAF --protein V600E \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[0:2][] | {id: .record.id, ids: .identity.ids})'
