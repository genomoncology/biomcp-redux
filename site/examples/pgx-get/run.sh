biomcp get pgx guidelines --gene DPYD --source cpic \
  | jq '.result.items[] | {guideline_name, drugs, guideline_url}'
