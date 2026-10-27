biomcp search study melanoma --limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.next[0].args | join(" "))'
