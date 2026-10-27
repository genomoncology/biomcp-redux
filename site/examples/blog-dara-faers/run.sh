for name in daraxonrasib RASONQUE; do
  biomcp search adverse-event "$name" --limit 3 \
    | jq -c --arg name "$name" \
      '{name: $name, status, total: .page.total, state: .sources[0].state}'
done
