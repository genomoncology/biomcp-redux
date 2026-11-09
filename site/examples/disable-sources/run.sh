biomcp --disable-sources noncommercial,licensed get gene BRAF \
  | jq '[.sources[] | {source, use, state, reason}]'
