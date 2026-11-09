biomcp get diagnostic P110020 --source fda-pma \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.header | {pma_number, trade_name, decision_date})'
