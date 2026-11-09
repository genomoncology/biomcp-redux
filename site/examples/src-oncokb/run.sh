biomcp get gene BRAF oncokb \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.next[0].args | join(" "))'
