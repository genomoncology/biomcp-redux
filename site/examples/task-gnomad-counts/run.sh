biomcp variant population CA118786 \
  | jq '.status as $s | .result.population.data
    | {status: $s, variant, release: .genome.frequency.dataset_version,
    genome: (.genome | {ac, an}), exome}'
