biomcp search adverse-event vemurafenib --limit 3 \
  | jq -r '.result.items[] | [.source, .id, .serious, .first_received] | @tsv'
