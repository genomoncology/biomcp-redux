biomcp get diagnostic K193046 --source fda-510k \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result | {k_number, device_name, decision_description, decision_date})'
