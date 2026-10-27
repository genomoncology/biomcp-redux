biomcp article text PMC7573235 --find E17K \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.content
      | {source, license: .data.license, matches: (.data.hits | length)})'
