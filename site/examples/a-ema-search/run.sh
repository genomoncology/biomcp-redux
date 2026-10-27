biomcp search drug Zelboraf --source ema --limit 1 \
  | jq '.result.items[] | {id, name}'
