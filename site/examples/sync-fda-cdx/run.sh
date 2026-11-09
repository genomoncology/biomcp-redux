biomcp sync fda-cdx \
  | jq '{status, result, sources: [.sources[] | {source, use, state}]}'
