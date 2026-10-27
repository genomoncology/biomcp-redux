biomcp get pgx guidelines --gene CYP2C19 --source cpic \
  | jq -r 'def cut: if length > 28 then .[:28] + "…" else . end;
    .result.items[] | [(.guideline_name | cut), .guideline_url] | @tsv'
