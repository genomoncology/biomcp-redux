for name in daraxonrasib RASONQUE; do
  biomcp search adverse-event "$name" --limit 3 | jq \
  --arg name "$name" '{name: $name, status, total: .page.total,
    sources: [.sources[] | {source, state}]}'
done
