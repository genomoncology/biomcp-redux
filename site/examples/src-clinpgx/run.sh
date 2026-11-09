biomcp get pgx --gene DPYD annotations --source clinpgx \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.items[0:2][]
      | {accessionId, variant: .location.displayName,
        level: .levelOfEvidence.term})'
