for drug in sotorasib adagrasib; do
  biomcp search drug "$drug" --source drugsfda --limit 1 | jq -r \
  --arg d "$drug" '.result.items[]
    | [$d, .identity.id, ([.products[]?.brand_name] | unique | join(","))]
    | @tsv'
done
