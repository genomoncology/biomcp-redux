biomcp get drug DDInter1925 interactions --source ddinter --section-limit 2 \
  | jq '{status, sources: [.sources[] | {source, state, reason}]},
    (.result.interactions.data.records[] | {partner: .partner.name, level})'
