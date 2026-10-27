biomcp --disable-sources noncommercial,restricted get gene BRAF | jq -c '[.sources[] | {source, use, state, reason}]'
