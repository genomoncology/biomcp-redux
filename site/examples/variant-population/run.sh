biomcp variant population CA123643 \
  | jq -c '{status, problems: (.problems | length), dataset: .result.population.data.dataset, exome: .result.population.data.exome | {data_type, ac, an, release: .frequency.dataset_version}}'
