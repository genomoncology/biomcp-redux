biomcp get gene BRAF oncokb | jq -c '{status, sources: [.sources[] | {source, state, reason}]}, (.next[0].args | join(" "))'
