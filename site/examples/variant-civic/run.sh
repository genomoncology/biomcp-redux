biomcp get variant CA123643 civic | jq -c '.result.candidates[0].sections.civic.data.profiles[0:2][] | {label, membership, record: .record.url}'
