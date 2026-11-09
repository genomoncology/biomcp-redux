biomcp get pgx --gene DPYD guidelines --source cpic \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[0:2][] | {guideline_name, drugs})'
