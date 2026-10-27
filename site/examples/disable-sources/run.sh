biomcp --disable-sources noncommercial,restricted get gene BRAF \
  | jq '[.sources[] | {source, use, state, reason}]'
