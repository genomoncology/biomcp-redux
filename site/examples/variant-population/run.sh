biomcp variant population CA123643 \
  | jq -c '{status, sources: [.sources[] | {source, state}], problems: [.problems[] | "\(.source) \(.field)"] | unique, dataset: .result.population.data.dataset, exome: .result.population.data.exome | {data_type, ac, an, release: .frequency.dataset_version}}'
